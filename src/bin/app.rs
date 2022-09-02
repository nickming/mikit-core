#[tokio::main]
pub async fn main() {
    let mut mikit = mikit_rust::kit::MiKit::new("mikit", "com.nickming").unwrap();
    let account = mikit.get_account().unwrap();
    println!("{:?}", account);
    let devices = mikit.fetch_devices().await.unwrap();
    println!("{:?}", devices);
}
