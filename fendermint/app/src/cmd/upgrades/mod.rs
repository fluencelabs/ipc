// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use anyhow::Context;
use fvm_ipld_blockstore::Blockstore;
use fvm_shared::chainid::ChainID;

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

    for (height, upgrade) in stage_upgrades() {
        upgrade_scheduler
            .add(Upgrade::new_by_id(
                FluenceChainId::Stage.into(),
                height,
                None,
                upgrade,
            ))
            .context(format!("upgrade for stage on height {height}"))?
    }

    for (height, upgrade) in dar_upgrades() {
        upgrade_scheduler
            .add(Upgrade::new_by_id(
                FluenceChainId::DAR.into(),
                height,
                None,
                upgrade,
            ))
            .context(format!("upgrade for dar on height {height}"))??
    }

    for (height, upgrade) in kras_upgrades() {
        upgrade_scheduler
            .add(Upgrade::new_by_id(
                FluenceChainId::Kras.into(),
                height,
                None,
                upgrade,
            ))
            .context(format!("upgrade for kras on height {height}"))??
    }

    Ok(upgrade_scheduler)
}

fn stage_upgrades<DB: Blockstore + 'static + Clone>() -> Vec<(BlockHeight, MigrationFunc<DB>)> {
    vec![
        // Deploy Batched Fluence Actor
        // 100137 = 17 Jun 9:00 AM UTC => 101000 = ~11:23 AM UTC
        (101_000, upgrade03::deploy_fluence_batched_actor),
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
