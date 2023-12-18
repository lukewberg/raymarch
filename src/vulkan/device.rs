#![allow(unused)]
use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::{VulkanError, VulkanLibrary};

pub fn get_physical_device(instance: Arc<Instance>) -> Option<Arc<PhysicalDevice>> {
    if let Ok(mut physical_device) = instance.enumerate_physical_devices() {
        match physical_device.next() {
            Some(device) => Some(device),
            None => None,
        }
    } else {
        None
    }
}

pub fn initialize_device(
    physical_device: Arc<PhysicalDevice>,
    flag: QueueFlags
) -> Option<(Arc<Device>, impl ExactSizeIterator<Item = Arc<Queue>>)> {
    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(flag)
        });

    if let Some(index) = queue_family_index {
        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: index as u32,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("Failed to create device!");
        Some((device, queues))
    } else {
        None
    }
}
