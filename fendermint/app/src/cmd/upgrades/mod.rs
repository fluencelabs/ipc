// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use anyhow::Context;
use fvm_ipld_blockstore::Blockstore;

use fendermint_vm_interpreter::fvm::upgrades::{Upgrade, UpgradeScheduler};

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

pub fn create_upgrade_scheduler<DB: Blockstore + 'static + Clone>(
) -> anyhow::Result<UpgradeScheduler<DB>> {
    let mut upgrade_scheduler = UpgradeScheduler::new();

    // Apply missing validator changes on Kras
    let kras_upgrade_01_height = 219500;
    let kras_upgrade_01 = Upgrade::new_by_id(
        (FluenceChainId::Kras as u64).into(),
        kras_upgrade_01_height,
        None, // do not change app_version this time
        upgrade01::store_missing_validator_changes,
    );
    upgrade_scheduler.add(kras_upgrade_01).context(format!(
        "upgrade01: store missing validator changes on Kras block {kras_upgrade_01_height}"
    ))?;

    // Apply missing validator changes on Kras
    let kras_upgrade_02_height = 507180;
    let kras_upgrade_02 = Upgrade::new_by_id(
        (FluenceChainId::Kras as u64).into(),
        kras_upgrade_02_height,
        None, // do not change app_version this time
        upgrade02::store_missing_validator_changes,
    );
    upgrade_scheduler.add(kras_upgrade_02).context(format!(
        "upgrade02: store missing validator changes on Kras block {kras_upgrade_02_height}"
    ))?;

    // Deploy Batched Fluence Actor
    // ==== Stage
    let upgrade_03_stage_height = 101_000; // 100137 = 17 Jun 9:00 AM UTC => 101000 = ~11:23 AM UTC
    let stage_upgrade_03 = Upgrade::new_by_id(
        (FluenceChainId::Stage as u64).into(),
        upgrade_03_stage_height,
        None, // do not change app_version this time
        upgrade03::deploy_fluence_batched_actor,
    );
    upgrade_scheduler.add(stage_upgrade_03).context(format!(
        "upgrade03: store missing validator changes on Stage block {upgrade_03_stage_height}"
    ))?;

    // ==== DAR
    let upgrade_03_dar_height = 600_000; // 592068 = 17 Jun 9:00 AM UTC => 600_000 = ~18 Jun 9:00 AM UTC
    let stage_upgrade_03 = Upgrade::new_by_id(
        (FluenceChainId::DAR as u64).into(),
        upgrade_03_dar_height,
        None, // do not change app_version this time
        upgrade03::deploy_fluence_batched_actor,
    );
    upgrade_scheduler.add(stage_upgrade_03).context(format!(
        "upgrade03: store missing validator changes on Dar block {upgrade_03_dar_height}"
    ))?;

    // ==== Kras
    // let upgrade_03_kras_height = 999999;
    // let kras_upgrade_03 = Upgrade::new_by_id(
    //     (FluenceChainId::Kras as u64).into(),
    //     upgrade_03_kras_height,
    //     None, // do not change app_version this time
    //     upgrade03::deploy_fluence_batched_actor,
    // );
    // upgrade_scheduler.add(kras_upgrade_03).context(format!(
    //     "upgrade03: store missing validator changes on Kras block {upgrade_03_kras_height}"
    // ))?;

    Ok(upgrade_scheduler)
}
