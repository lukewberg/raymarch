use crate::vulkan::devices::{get_instance, get_devices};


#[test]
fn test_instance() {
    let instance = get_instance().unwrap();
    println!("{}", instance.api_version());
}

#[test]
fn test_devices() {
    let instance = get_instance().unwrap();
    let physical_device = get_devices(instance.clone());
    println!("Vulkan-compatible device found: {:?}", physical_device.properties().device_name);
    for family in physical_device.queue_family_properties() {
        println!("Found a queue family with {:?} {:?} queue(s)", family.queue_count, family.queue_flags);
    }
}
