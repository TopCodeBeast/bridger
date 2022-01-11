use substrate_subxt::ClientBuilder;

use client_pangolin::darwinia::runtime::DarwiniaRuntime;
use client_pangolin::frame::sudo::KeyStoreExt;
use client_pangolin::frame::technical_committee::MembersStoreExt;
use support_common::config::{Config, Names};
use support_terminal::output;

use crate::bridge::PangolinRopstenConfig;

pub async fn handle_keys() -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let config_darwinia = bridge_config.darwinia;

    let client = ClientBuilder::<DarwiniaRuntime>::new()
        .set_url(config_darwinia.endpoint)
        .build()
        .await?;
    let sudo = client.key(None).await?;
    // let sudo_ss58 = sudo.to_string();
    let technical_committee_members = client.members(None).await?;

    let msgs = vec![
        format!("sudo key: {:?}", sudo),
        format!(
            "technical committee members: {:?}",
            technical_committee_members
        ),
    ];

    output::output_text(msgs.join("\n"));
    Ok(())
}