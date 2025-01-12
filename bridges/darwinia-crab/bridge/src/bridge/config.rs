use serde::{Deserialize, Serialize};

use component_subscan::SubscanConfig;
use support_common::error::BridgerError;

use crate::types::{ChainInfo, HexLaneId, PrometheusParamsInfo};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaCrabConfig {
    pub darwinia: ChainInfoConfig,
    pub crab: ChainInfoConfig,
    pub relay: RelayConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darwinia_subscan: Option<SubscanConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crab_subscan: Option<SubscanConfig>,
    pub task: TaskConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskConfig {
    pub interval_update_fee: u64,
    pub update_fee_strategy: UpdateFeeStrategyType,
}

#[derive(Clone, Debug, Serialize, Deserialize, strum::EnumString)]
pub enum UpdateFeeStrategyType {
    Nothing,
    Crazy,
    Reasonable,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelayConfig {
    /// Hex-encoded lane identifiers that should be served by the complex relay.
    pub lanes: Vec<HexLaneId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_darwinia: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_crab: Option<String>,
    #[serde(default)]
    pub prometheus_params: PrometheusParamsInfo,
    /// If passed, only mandatory headers (headers that are changing the GRANDPA authorities set)
    /// are relayed.
    pub only_mandatory_headers: bool,
    /// Create relayers fund accounts on both chains, if it does not exists yet.
    pub create_relayers_fund_accounts: bool,
    /// The SURI of secret key to use when transactions are submitted to the darwinia node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darwinia_messages_pallet_owner: Option<String>,
    /// The password for the SURI of secret key to use when transactions are submitted to the darwinia node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darwinia_messages_pallet_owner_password: Option<String>,
    /// The SURI of secret key to use when transactions are submitted to the crab node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crab_messages_pallet_owner: Option<String>,
    /// The password for the SURI of secret key to use when transactions are submitted to the crab node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crab_messages_pallet_owner_password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfoConfig {
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<String>,
    #[serde(skip)]
    pub secure: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_password: Option<String>,
    /// Transactions mortality period, in blocks. MUST be a power of two in [4; 65536] range. MAY NOT be larger than `BlockHashCount` parameter of the chain system module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions_mortality: Option<u32>,
}

impl ChainInfoConfig {
    fn host_port(&self) -> color_eyre::Result<(bool, String, u16)> {
        if self.endpoint.find("ws://").unwrap_or(usize::MAX) != 0
            && self.endpoint.find("wss://").unwrap_or(usize::MAX) != 0
        {
            return Err(BridgerError::Custom(
                "The entrypoint isn't websocket protocol".to_string(),
            )
            .into());
        }
        let secure = self.endpoint.starts_with("wss://");
        let endpoint = self
            .endpoint
            .replace(if secure { "wss://" } else { "ws://" }, "")
            .replace('/', "")
            .replace(' ', "");
        let host_port = endpoint.split(':').collect::<Vec<&str>>();
        let host = host_port.get(0).unwrap_or(&"127.0.0.1");
        let port = host_port
            .get(1)
            .unwrap_or(if secure { &"443" } else { &"80" });
        Ok((secure, host.to_string(), port.parse::<u16>()?))
    }

    pub fn to_chain_info(&self) -> color_eyre::Result<ChainInfo> {
        self.to_chain_info_with_expect_signer(None)
    }

    pub fn to_chain_info_with_expect_signer(
        &self,
        except_signer: Option<String>,
    ) -> color_eyre::Result<ChainInfo> {
        let host_port = self.host_port()?;
        Ok(ChainInfo {
            secure: host_port.0,
            host: host_port.1,
            port: host_port.2,
            signer: except_signer.or_else(|| self.signer.clone()),
            signer_password: self.signer_password.clone(),
            transactions_mortality: Some(256),
        })
    }
}
