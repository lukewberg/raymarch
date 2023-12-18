use std::sync::Arc;

use vulkano::{VulkanLibrary, instance::{InstanceCreateInfo, InstanceCreateFlags, Instance}, Validated, VulkanError};

pub mod device;

pub fn get_instance() -> Result<Arc<Instance>, Validated<VulkanError>> {
    let library = VulkanLibrary::new().expect("No local Vulkan library/DLL");
    let instance_info = InstanceCreateInfo {
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        ..Default::default()
    };

    let instance = Instance::new(library, instance_info)?;

    Ok(instance)
}