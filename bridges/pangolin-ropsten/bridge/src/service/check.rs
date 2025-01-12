use std::time::SystemTime;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use client_pangolin::component::DarwiniaSubxtComponent;
use component_state::state::BridgeState;
use component_thegraph_liketh::component::TheGraphLikeEthComponent;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::PangolinRopstenTask;
use crate::bridge::TaskConfig;
use crate::bridge::{PangolinRopstenBus, PangolinRopstenConfig};
use crate::helpers;

/// Check service
#[derive(Debug)]
pub struct CheckService {
    _greet: Lifeline,
}

impl BridgeService for CheckService {}

impl Service for CheckService {
    type Bus = PangolinRopstenBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::name());
        let tracker = Tracker::new(microkv, "scan.ropsten.check");

        // scan task
        let _greet = Self::try_task(
            &format!("{}-service-check", PangolinRopstenTask::name()),
            async move {
                start(tracker.clone()).await;
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn start(tracker: Tracker) {
    while let Err(err) = run(&tracker).await {
        tracing::error!(
            target: "pangolin-ropsten",
            "ropsten check err {:#?}",
            err
        );
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

async fn run(tracker: &Tracker) -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-ropsten",
        "ROPSTEN CHECK SERVICE RESTARTING..."
    );
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let task_config: TaskConfig = bridge_config.task;

    let thegraph_liketh = TheGraphLikeEthComponent::component(bridge_config.thegraph)?;

    // Darwinia client
    let darwinia = DarwiniaSubxtComponent::component(bridge_config.darwinia).await?;

    let mut timing = SystemTime::now();
    loop {
        let from = tracker.current().await?;
        let limit = 1usize;

        tracing::trace!(
            target: "pangolin-ropsten",
            "[ropsten] Track check block: {} and limit: {}",
            from,
            limit
        );
        let txs = thegraph_liketh
            .query_transactions(from as u64, limit as u32)
            .await?;
        if txs.is_empty() {
            tracing::info!(
                target: "pangolin-ropsten",
                "[ropsten] All transactions checked"
            );
            tokio::time::sleep(std::time::Duration::from_secs(task_config.interval_check)).await;
            continue;
        }
        let tx = txs.get(0).unwrap();

        let verified = match helpers::is_verified(&darwinia, tx).await {
            Ok(v) => v,
            Err(e) => {
                if let Some(substrate_subxt::Error::Rpc(_)) =
                    e.downcast_ref::<substrate_subxt::Error>()
                {
                    return Err(e);
                }
                let err_msg = format!("{:?}", e).to_lowercase();
                if err_msg.contains("restart") {
                    return Err(e);
                }
                tracing::error!(
                    target: "pangolin-ropsten",
                    "Failed verified redeem. [{}]: {}. {:?}",
                    tx.block_number,
                    tx.block_hash,
                    e
                );
                false
            }
        };
        if verified {
            tracker.finish(tx.block_number as usize)?;
            timing = SystemTime::now();
            continue;
        }

        if let Ok(elapsed) = timing.elapsed() {
            let secs = elapsed.as_secs();
            if secs >= task_config.check_timeout {
                tracker.finish(tx.block_number as usize)?;
                // todo: check timeout, skip thi transaction, write log
                continue;
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(task_config.interval_check)).await;
    }
}
