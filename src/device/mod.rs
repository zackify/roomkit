mod generic_device;
use crate::integrations::HueBridge;
pub use generic_device::{GenericDevice, GenericResult};

pub struct Device<Type> {
  pub device: Type,
}

impl Device<HueBridge> {
  pub fn new<Type>(device: Type) -> Device<Type> {
    Device::<Type> { device: device }
  }

  pub fn name(&self) -> String {
    self.device.name()
  }

  pub fn initialize(&mut self) -> GenericResult {
    self.device.initialize()
  }
}
