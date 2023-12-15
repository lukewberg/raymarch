use vulkano::Version;

use crate::vulkan::devices::{get_instance, get_devices};

#[test]
fn test_instance() {
    let instance = get_instance().unwrap();
    println!("{}", instance.api_version());
}

#[test]
fn test_devices() {
    let instance = get_instance().unwrap();
    let device = get_devices(instance.clone());
    println!("Vulkan-compatible device found: {:?}", device.properties().device_name);
}
