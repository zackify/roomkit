pub type GenericResult = Result<(), String>;

pub trait GenericDevice {
  fn initialize(&mut self) -> GenericResult;
}
