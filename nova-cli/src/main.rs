// Minimal runnable CLI entrypoint for demonstration.
fn main() {
    // Delegate to nova-core implementation.
    nova_core::wallet::send(10);
}
