use napi::Status;

use super::IntoRawConfig;

pub type StyledComponentsConfigNapi = String;

impl IntoRawConfig<modern_swc_plugins_collection::plugin_styled_components::Config>
  for StyledComponentsConfigNapi
{
  fn into_raw_config(
    self,
    _env: napi::Env,
  ) -> napi::Result<modern_swc_plugins_collection::plugin_styled_components::Config> {
    serde_json::from_str(&self).map_err(|_| {
      napi::Error::new(
        Status::InvalidArg,
        "invalid styled components options".into(),
      )
    })
  }
}
