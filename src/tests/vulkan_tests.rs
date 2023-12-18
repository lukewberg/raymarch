use std::sync::Arc;

use vulkano::device::{Queue, QueueFlags};

use crate::vulkan::device::{get_physical_device, initialize_device};
use crate::vulkan::get_instance;

#[test]
fn test_instance() {
    let instance = get_instance().unwrap();
    println!("{}", instance.api_version());
}

#[test]
fn test_physical_device() {
    let instance = get_instance().unwrap();
    let physical_device =
        get_physical_device(instance.clone()).expect("No Vulkan-compatible device found!");
    println!(
        "Vulkan-compatible device found: {:?} with {:?} \n",
        physical_device.properties().device_name,
        physical_device.memory_properties().memory_heaps
    );
    for family in physical_device.queue_family_properties() {
        println!(
            "Found a queue family with {:?} {:?} queue(s)",
            family.queue_count, family.queue_flags
        );
    }
}

#[test]
fn test_initialize_device() {
    let instance = get_instance().unwrap();
    let physical_device =
        get_physical_device(instance.clone()).expect("No Vulkan-compatible device found!");
    let (device, mut queues) = initialize_device(physical_device, QueueFlags::COMPUTE).unwrap();
    println!(
        "Device {:?} with queues: {:#?}",
        device.physical_device().properties().device_name,
        queues.collect::<Vec<Arc<Queue>>>()
    );
}
