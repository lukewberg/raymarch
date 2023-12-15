#![allow(unused)]
use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::{Instance, InstanceCreateInfo, InstanceCreateFlags};
use vulkano::VulkanLibrary;

pub fn get_instance() -> Option<Arc<Instance>> {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let mut instance_info = InstanceCreateInfo::default();
    instance_info.flags = InstanceCreateFlags::ENUMERATE_PORTABILITY;

    match Instance::new(library, instance_info) {
        Ok(instance) => {
            return Some(instance)
        },
        Err(err) => {
            println!("{:?}", err);
        },
    };
    None
    // let instance =
    //     Instance::new(library, instance_info).expect("failed to create instance");
    // instance
}

pub fn get_devices(instance: Arc<Instance>) -> Arc<PhysicalDevice> {
    instance.enumerate_physical_devices().expect("Unable to enumerate devices!").next().expect("No devices available!")
}
