use crate::action_types::Action;
use crate::integrations::Integration;

pub struct DidEnter {
  room: String,
  action: Action,
}

impl DidEnter {
  pub fn run(&mut self, integrations: &mut Integration) {
    (self.action.callback)(integrations);
  }
}
