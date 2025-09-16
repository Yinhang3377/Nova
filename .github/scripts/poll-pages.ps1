# Poll GitHub Pages and the public URL until active
# Usage: powershell -ExecutionPolicy Bypass -File .github/scripts/poll-pages.ps1 -RepoOwner <owner> -RepoName <repo> -IntervalMinutes 15
param(
    [string]$RepoOwner = "Yinhang3377",
    [string]$RepoName = "Nova",
    [int]$IntervalMinutes = 15,
    [int]$IntervalSeconds = 0,
    [string]$Token = $null
)

# If no token provided explicitly, read from environment variable GITHUB_TOKEN for safer invocation
if ([string]::IsNullOrEmpty($Token)) {
    $envToken = $env:GITHUB_TOKEN
    if (-not [string]::IsNullOrEmpty($envToken)) {
        $Token = $envToken
        Write-Host "Using GITHUB_TOKEN from environment for authenticated API calls."
    }
}

function Get-PagesApi {
    param($owner, $repo, $token)
    $url = "https://api.github.com/repos/$owner/$repo/pages"
    $headers = @{ "User-Agent" = "poll-pages-script" }
    if ($token) { $headers["Authorization"] = "token $token" }

    # First attempt: classic PAT style (token ...). If we get 401 and a token is present,
    # retry with Bearer for fine-grained tokens which sometimes require that scheme.
    try {
        $resp = Invoke-RestMethod -Uri $url -Headers $headers -Method Get -ErrorAction Stop
        return @{ Ok = $true; Json = $resp; Scheme = "token" }
    } catch {
        # Try to extract numeric status and response body if present
        $status = $null
        $body = $null
        if ($_.Exception.Response -ne $null) {
            $status = $_.Exception.Response.StatusCode.Value__
            try {
                $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
                $body = $reader.ReadToEnd()
            } catch {
                $body = $null
            }
        }

        if ($status -eq 401 -and $token) {
            # Retry with Bearer scheme
            $headers["Authorization"] = "Bearer $token"
            try {
                $resp2 = Invoke-RestMethod -Uri $url -Headers $headers -Method Get -ErrorAction Stop
                return @{ Ok = $true; Json = $resp2; Scheme = "Bearer" }
            } catch {
                $status2 = $null
                $body2 = $null
                if ($_.Exception.Response -ne $null) {
                    $status2 = $_.Exception.Response.StatusCode.Value__
                    try {
                        $reader2 = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
                        $body2 = $reader2.ReadToEnd()
                    } catch {
                        $body2 = $null
                    }
                }
                return @{ Ok = $false; Error = $status2; Body = $body2; Tried = @("token","Bearer") }
            }
        }

        if ($status -ne $null) {
            return @{ Ok = $false; Error = $status; Body = $body }
        }
        return @{ Ok = $false; Error = "Unknown" }
    }
}

function Test-PublicUrl {
    param($url)
    try {
        $req = Invoke-WebRequest -Uri $url -Method Head -UseBasicParsing -ErrorAction Stop
        return @{ Ok = $true; Status = $req.StatusCode }
    } catch {
        if ($null -ne $_.Exception.Response) {
            return @{ Ok = $false; Status = $_.Exception.Response.StatusCode.Value__ }
        }
        return @{ Ok = $false; Status = "Unknown" }
    }
}

$publicUrl = "https://$($RepoOwner.ToLower()).github.io/$RepoName/"
Write-Host "Starting Pages poller for repository $RepoOwner/$RepoName"
Write-Host "Public URL: $publicUrl"

# Single-check mode: if IntervalMinutes <= 0 then perform one check and exit
if ($IntervalMinutes -le 0) {
    $now = Get-Date -Format o
    Write-Host "[$now] Single-check mode: Checking Pages API..."
    $apiResult = Get-PagesApi -owner $RepoOwner -repo $RepoName -token $Token
    if ($apiResult.Ok) {
        Write-Host "Pages API reports a site:
$($apiResult.Json | ConvertTo-Json -Depth 5)"
        $pub = Test-PublicUrl -url $publicUrl
        Write-Host "Public URL status: $($pub.Status)"
    } else {
        Write-Host "Pages API not configured (status: $($apiResult.Error))."
    }
    Write-Host "Done (single-check)."
    return
}

while ($true) {
    $now = Get-Date -Format o
    Write-Host "[$now] Checking Pages API..."
    $apiResult = Get-PagesApi -owner $RepoOwner -repo $RepoName -token $Token
    if ($apiResult.Ok) {
        Write-Host "Pages API reports a site:
$($apiResult.Json | ConvertTo-Json -Depth 5)"
        Write-Host "Checking public URL: $publicUrl"
    $pub = Test-PublicUrl -url $publicUrl
        Write-Host "Public URL status: $($pub.Status)"
        if ($pub.Ok -and $pub.Status -eq 200) {
            Write-Host "Pages site is live. Exiting poller."
            break
        }
    } else {
        Write-Host "Pages API not configured (status: $($apiResult.Error)). If this is 429/403 consider supplying a GitHub token to avoid rate limits."
    }

    if ($IntervalSeconds -gt 0) {
        Write-Host "Sleeping for $IntervalSeconds second(s)..."
        Start-Sleep -Seconds $IntervalSeconds
    } else {
        Write-Host "Sleeping for $IntervalMinutes minute(s)..."
        Start-Sleep -Seconds ($IntervalMinutes * 60)
    }
}

Write-Host "Done."