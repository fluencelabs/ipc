// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::multihash::Code;
use num_traits::Zero;

use fendermint_rocksdb::blockstore::NamespaceBlockstore;
use fendermint_vm_actor_interface::fluence_batched::FLUENCE_BATCHED_ACTOR_ID;
use fendermint_vm_actor_interface::EMPTY_ARR;
use fendermint_vm_interpreter::fvm::state::FvmExecState;
use fendermint_vm_interpreter::fvm::upgrades::Upgrade;
use fendermint_vm_interpreter::fvm::upgrades::UpgradeScheduler;

use fvm_ipld_blockstore::Block;
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_encoding::CborStore;
use fvm_shared::econ::TokenAmount;
use fvm_shared::state::ActorState;
use fvm_shared::IPLD_RAW;

use crate::cmd::upgrades::CHAIN_ID;

static FLUENCE_BATCHED_WASM_BIN: &[u8] =
    include_bytes!(".../upgrade03/fendermint_actor_fluence_batched.wasm");

fn upgrade_wasm_actor_func(state: &mut FvmExecState<NamespaceBlockstore>) -> anyhow::Result<()> {
    let state_tree = state.state_tree_mut();

    // store the new wasm code in the blockstore and get the new code cid
    //
    let new_code_cid = state_tree.store().put(
        multihash::Code::Blake2b256,
        &Block {
            codec: IPLD_RAW,
            data: FLUENCE_BATCHED_WASM_BIN,
        },
    )?;
    tracing_log::info!("fluence batched actor code_cid: {:?}", new_code_cid);

    let new_empty_state = state_tree.store().put_cbor(&EMPTY_ARR, Code::Blake2b256)?;

    // update the actor state in the state tree
    state_tree.set_actor(
        FLUENCE_BATCHED_ACTOR_ID,
        ActorState {
            code: new_code_cid,
            state: new_empty_state,
            // number of times this actor was called
            // it's new one, so set zero
            sequence: 0,
            balance: TokenAmount::zero(),
            delegated_address: None,
        },
    );

    Ok(())
}

pub(crate) fn upgrade_actor<DB: Blockstore + 'static + Clone>(
    upgrade_scheduler: &mut UpgradeScheduler<DB>,
    block_height: u64,
) -> anyhow::Result<()> {
    upgrade_scheduler.add(Upgrade::new_by_id(
        CHAIN_ID.into(),
        block_height,
        None,
        |state| upgrade_wasm_actor_func(state),
    ))
}
