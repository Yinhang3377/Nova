pub fn send(amount: u64) {
    println!("钱包发送 币: {}", amount);
    crate::bridge::lock(amount);
}
