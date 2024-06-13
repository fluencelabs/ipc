// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

mod upgrade01;
mod upgrade02;
mod upgrade03;

use fendermint_vm_interpreter::fvm::upgrades::UpgradeScheduler;
use fvm_ipld_blockstore::Blockstore;
use std::env;

const CHAIN_ID: u64 = 1622562509754216;

pub fn create_upgrade_scheduler<DB: Blockstore + 'static + Clone>(
) -> anyhow::Result<UpgradeScheduler<DB>> {
    let mut upgrade_scheduler = UpgradeScheduler::new();

    // applied missing validator changes
    let target_height = {
        let h = env::var("FLUENCE_UPGRADE_01_HEIGHT").unwrap_or(String::from("219500"));
        h.parse().expect("unable to parse upgrade height")
    };
    upgrade01::store_missing_validator_changes(&mut upgrade_scheduler, target_height)?;

    // applied missing validator changes
    let target_height = {
        let h = env::var("FLUENCE_UPGRADE_02_HEIGHT").unwrap_or(String::from("507180")); // ~ 30 May 16:15 UTC
        h.parse().expect("unable to parse upgrade 2 height")
    };
    upgrade02::store_missing_validator_changes(&mut upgrade_scheduler, target_height)?;

    // upgrade Fluence Actor to a new one
    let target_height = {
        let h = env::var("FLUENCE_UPGRADE_03_HEIGHT").unwrap_or(String::from("507180")); // ~ 30 May 16:15 UTC
        h.parse().expect("unable to parse upgrade 3 height")
    };
    upgrade02::store_missing_validator_changes(&mut upgrade_scheduler, target_height)?;

    Ok(upgrade_scheduler)
}
