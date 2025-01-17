use crate::config::{format_graphql, UpdateRegistry};
use crate::PartialWapmConfig;

/// Login to a registry and save the token associated with it.
///
/// Also sets the registry as the currently active registry to provide a better UX.
pub fn login_and_save_token(
    #[cfg(test)] test_name: &str,
    registry: &str,
    token: &str,
) -> Result<(), anyhow::Error> {
    let registry = format_graphql(registry);
    #[cfg(test)]
    let mut config = PartialWapmConfig::from_file(test_name).map_err(|e| anyhow::anyhow!("{e}"))?;
    #[cfg(not(test))]
    let mut config = PartialWapmConfig::from_file().map_err(|e| anyhow::anyhow!("{e}"))?;
    config.registry.set_current_registry(&registry);
    config.registry.set_login_token_for_registry(
        &config.registry.get_current_registry(),
        token,
        UpdateRegistry::Update,
    );
    #[cfg(test)]
    let path =
        PartialWapmConfig::get_file_location(test_name).map_err(|e| anyhow::anyhow!("{e}"))?;
    #[cfg(not(test))]
    let path = PartialWapmConfig::get_file_location().map_err(|e| anyhow::anyhow!("{e}"))?;
    config.save(&path)?;
    let username = crate::utils::get_username_registry_token(&registry, token);
    if let Some(s) = username.ok().and_then(|o| o) {
        println!("Login for WAPM user {:?} saved", s);
    } else {
        println!("Login for WAPM user saved");
    }
    Ok(())
}
