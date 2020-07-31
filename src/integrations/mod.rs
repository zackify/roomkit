mod hue_bridge;
pub use hue_bridge::HueBridge;

pub enum Integration {
  Hue(HueBridge),
  Test(),
}
