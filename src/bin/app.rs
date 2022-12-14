#[tokio::main]
pub async fn main() {
    let mut mikit = mikit_rust::kit::MiKit::default();
    let devices: Vec<String> = mikit
        .fetch_devices()
        .await
        .unwrap()
        .iter()
        .map(|x| x.model.clone())
        .collect();
    println!("{:?}", devices);
    // let mut devices_info = vec![];
    // devices_info.push(DeviceProperties::new_get_properties("xxx", 2, 1));
    // let properties = mikit.get_device_properties(&devices_info).await.unwrap();
    // println!("{:?}", properties);

    // let mut set_device_properties = vec![];
    // set_device_properties.push(DeviceProperties::new_set_properties(
    //     "xxx",
    //     2,
    //     1,
    //     Value::Bool(true),
    // ));
    // mikit
    //     .set_device_properties(&set_device_properties)
    //     .await
    //     .unwrap();
}
