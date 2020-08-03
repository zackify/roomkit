use crate::home::Home;
use crate::integrations::HueBridge;

// try to make this check if someone is in a room then do something else
// add second action type that takes a room struct, that you pass in to did enter
// or did leave
// register
pub fn test_action(home: &mut Home) {
  if let Some(hue) = home.integration::<HueBridge>() {
    print!("in action {:?}", hue.light_ids)
  }
}
