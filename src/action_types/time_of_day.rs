use crate::action_types::Action;
use crate::integrations::Integration;

pub struct TimeOfDay {
  time: String,
  action: Action,
}

impl TimeOfDay {
  pub fn run(&mut self, integrations: &mut Integration) {
    (self.action.callback)(integrations);
  }
}
