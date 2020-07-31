use crate::device::Device;

pub struct Home {
  pub devices: Vec<Device>,
}

impl Home {
  pub fn initialize(&mut self) {
    println!("Bringing {:?} devices online...", self.devices.len());

    self
      .devices
      .iter_mut()
      .for_each(|device| match device.initialize() {
        Ok(_) => {
          println!("Initialized {:?}", device.name());
        }
        Err(e) => println!("Failed to initialize {:?}: {:?}", device.name(), e),
      });
  }
}
