pub type GenericResult = Result<(), String>;

pub trait GenericDevice {
  fn name(&self) -> String;
  fn initialize(&mut self) -> GenericResult;
}
