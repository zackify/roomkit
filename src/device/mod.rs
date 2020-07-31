mod generic_device;
pub use generic_device::{GenericDevice, GenericResult};

pub struct Device {
  pub device: Box<dyn GenericDevice>,
}

impl Device {
  pub fn new(device: impl GenericDevice + 'static) -> Device {
    Device {
      device: Box::new(device),
    }
  }

  pub fn name(&self) -> String {
    self.device.name()
  }

  pub fn initialize(&mut self) -> GenericResult {
    self.device.initialize()
  }
}
