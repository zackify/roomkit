mod did_enter;
mod did_leave;
mod time_of_day;
use crate::integrations::Integration;

pub struct Action {
  pub callback: Box<dyn FnMut(&mut Integration)>,
}
