use crate::home::Home;
use crate::integrations::Integration;

//try to make this check if someone is in a room then do something else
pub fn test_action(home: &mut Home) {
  home
    .integrations
    .iter_mut()
    .for_each(|integration| match integration {
      Integration::HueBridge(hue) => print!("in action {:?}", hue.light_ids),
      // => (),
    });
}
