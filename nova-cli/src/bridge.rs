pub fn lock(amount: u64) {
    println!("桥锁住 币: {}", amount);
    crate::consensus::propose();
}
