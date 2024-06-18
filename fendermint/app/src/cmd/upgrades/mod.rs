// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use anyhow::Context;
use fvm_ipld_blockstore::Blockstore;
use fvm_shared::chainid::ChainID;

use crate::cmd::upgrades::FluenceChainId::{Kras, Stage, DAR};
use fendermint_vm_interpreter::fvm::state::snapshot::BlockHeight;
use fendermint_vm_interpreter::fvm::upgrades::{MigrationFunc, Upgrade, UpgradeScheduler};

mod upgrade01;
mod upgrade02;
mod upgrade03;

#[repr(u64)]
pub enum FluenceChainId {
    // cast chain-id --rpc-url https://ipc.stage.fluence.dev
    Stage = 2182032320410279,
    // cast chain-id --rpc-url https://ipc.dar.fluence.dev
    DAR = 2358716091832359,
    // cast chain-id --rpc-url https://ipc.kras.fluence.dev
    Kras = 1622562509754216,
}

impl From<FluenceChainId> for ChainID {
    fn from(value: FluenceChainId) -> Self {
        (value as u64).into()
    }
}

pub fn create_upgrade_scheduler<DB: Blockstore + 'static + Clone>(
) -> anyhow::Result<UpgradeScheduler<DB>> {
    let mut upgrade_scheduler = UpgradeScheduler::new();

    schedule_upgrades(stage_upgrades(), Stage, "stage", &mut upgrade_scheduler)?;
    schedule_upgrades(dar_upgrades(), DAR, "DAR", &mut upgrade_scheduler)?;
    schedule_upgrades(kras_upgrades(), Kras, "Kras", &mut upgrade_scheduler)?;

    Ok(upgrade_scheduler)
}

fn stage_upgrades<DB: Blockstore + 'static + Clone>() -> Vec<(BlockHeight, MigrationFunc<DB>)> {
    vec![
        // Deploy Batched Fluence Actor
        // 101082 = 11:40 AM UTC => 101177 = ~11:56 AM UTC
        (101177, upgrade03::deploy_fluence_batched_actor),
    ]
}

fn dar_upgrades<DB: Blockstore + 'static + Clone>() -> Vec<(BlockHeight, MigrationFunc<DB>)> {
    vec![
        // Deploy Batched Fluence Actor
        // 592068 = 17 Jun 9:00 AM UTC => 600_000 = ~18 Jun 9:00 AM UTC
        (600_000, upgrade03::deploy_fluence_batched_actor),
    ]
}

fn kras_upgrades<DB: Blockstore + 'static + Clone>() -> Vec<(BlockHeight, MigrationFunc<DB>)> {
    vec![
        // Apply missing validator changes on Kras
        (219500, upgrade01::store_missing_validator_changes),
        // Apply missing validator changes on Kras
        (507180, upgrade02::store_missing_validator_changes),
        // (999999, upgrade03::deploy_fluence_batched_actor)
    ]
}

fn schedule_upgrades<DB: Blockstore + 'static + Clone>(
    upgrades: Vec<(BlockHeight, MigrationFunc<DB>)>,
    chain_id: FluenceChainId,
    chain_name: &'static str,
    scheduler: &mut UpgradeScheduler<DB>,
) -> anyhow::Result<()> {
    let chain_id: ChainID = chain_id.into();
    for (height, upgrade) in upgrades {
        log::info!("Scheduling an upgrade for {chain_name} on height {height}");
        scheduler
            .add(Upgrade::new_by_id(chain_id, height, None, upgrade))
            .context(format!("upgrade for {chain_name} on height {height}"))?
    }
    Ok(())
}
