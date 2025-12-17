// src/setup/single_instance.rs
use single_instance::SingleInstance;

pub fn check_single_instance() -> Option<SingleInstance> {
    let instance = SingleInstance::new("docsort_single_instance").unwrap();
    if !instance.is_single() {
        None
    } else {
        Some(instance)
    }
}
