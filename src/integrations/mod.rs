mod hue_bridge;
use crate::device::{GenericDevice, GenericResult};
pub use hue_bridge::HueBridge;

pub enum Integration {
  HueBridge(HueBridge),
}

impl GenericDevice for Integration {
  fn name(&self) -> String {
    match self {
      Integration::HueBridge(hue) => hue.name(),
      //_ => "".into(),
    }
  }
  fn initialize(&mut self) -> GenericResult {
    match self {
      Integration::HueBridge(hue) => hue.initialize(),
      //_ => Ok(()),
    }
  }
}
