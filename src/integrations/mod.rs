mod hue_bridge;
pub use hue_bridge::HueBridge;

pub type GenericResult = Result<(), String>;

pub trait GenericIntegration {
  fn name(&self) -> String;
  fn initialize(&mut self);
}
