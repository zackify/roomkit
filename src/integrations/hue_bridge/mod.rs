use crate::integrations::GenericIntegration;
use reqwest;
mod toggle_light;
use serde_derive::Deserialize;
use std::any::Any;
use std::collections::HashMap;
pub use toggle_light::toggle_light;

#[derive(Debug)]
pub struct HueBridge {
  /*
    You need to create a new hue username, and use the full url
    ex: http://192.168.86.30/api/ZhcYv5Z9bHRnPPXP9Hu2yLt9OgVYswm1YtydfGX4

    To create a user, follow the Hue documentation here:
    https://developers.meethue.com/develop/hue-api/7-configuration-api/
  */
  pub bridge_url: String,
  //The light id's that your hue bridge gives us
  pub light_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct HueState {
  on: bool,
}

#[derive(Debug, Deserialize)]
pub struct HueLightsResponse {
  state: HueState,
}
type LightResponse = HashMap<String, HueLightsResponse>;

impl HueBridge {
  pub fn create(bridge_url: String) -> Box<dyn Any> {
    let mut instance = HueBridge {
      light_ids: None,
      bridge_url: bridge_url,
    };
    instance.initialize();

    Box::new(instance)
  }
  pub fn get_lights(&mut self) -> reqwest::Result<LightResponse> {
    let lights_endpoint = format!("{}{}", &self.bridge_url, "/lights");
    let body: LightResponse = reqwest::blocking::get(&lights_endpoint)?.json()?;

    let light_ids: Vec<String> = body.keys().cloned().collect();
    self.light_ids = Some(light_ids);
    Ok(body)
  }
}

impl GenericIntegration for HueBridge {
  fn name(&self) -> String {
    "Hue Bridge".to_string()
  }
  fn initialize(&mut self) {
    match self.get_lights() {
      Err(e) => println!("Unable to request hue bridge {:?}", e),
      _ => (),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::integrations::GenericIntegration;
  use crate::integrations::HueBridge;

  #[test]
  fn test_creating_hue_bridge() {
    let hue = HueBridge {
      light_ids: None,
      bridge_url: "http://192.168.86.30/api/test".into(),
    };

    assert_eq!(hue.name(), "Hue Bridge");
    assert_eq!(hue.light_ids, None);
    assert_eq!(
      hue.bridge_url,
      String::from("http://192.168.86.30/api/test")
    );
  }
}
