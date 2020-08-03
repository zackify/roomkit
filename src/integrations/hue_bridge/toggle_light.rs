use crate::home::Home;
use crate::integrations::HueBridge;

// try to make this check if someone is in a room then do something else
// add second action type that takes a room struct, that you pass in to did enter
// or did leave
// register
pub fn toggle_light(home: &mut Home) {
  // grab our active hue bridge integration and print the light ids
  if let Some(hue) = home.integration::<HueBridge>() {
    println!("in action {:?}", hue.light_ids)
  }
}
