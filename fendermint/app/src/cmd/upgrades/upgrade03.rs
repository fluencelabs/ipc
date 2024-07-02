use cid::multihash::Code;
use num_traits::Zero;

use fendermint_vm_actor_interface::fluence_batched::FLUENCE_BATCHED_ACTOR_ID;
use fendermint_vm_actor_interface::EMPTY_ARR;
use fendermint_vm_interpreter::fvm::state::FvmExecState;

use fvm_ipld_blockstore::Block;
use fvm_ipld_blockstore::Blockstore;
use fvm_ipld_encoding::CborStore;
use fvm_shared::econ::TokenAmount;
use fvm_shared::state::ActorState;
use fvm_shared::IPLD_RAW;

static FLUENCE_BATCHED_WASM_BIN: &[u8] =
    include_bytes!("./upgrade03/fendermint_actor_fluence_batched.wasm");

pub fn deploy_fluence_batched_actor<DB: Blockstore + 'static + Clone>(
    state: &mut FvmExecState<DB>,
) -> anyhow::Result<()> {
    let state_tree = state.state_tree_mut();

    // store the new wasm code in the blockstore and get the new code cid
    let new_code_cid = state_tree.store().put(
        multihash::Code::Blake2b256,
        &Block {
            codec: IPLD_RAW,
            data: FLUENCE_BATCHED_WASM_BIN,
        },
    )?;
    log::info!("fluence batched actor code_cid: {:?}", new_code_cid);

    let new_empty_state = state_tree.store().put_cbor(&EMPTY_ARR, Code::Blake2b256)?;

    // register new Fluence batched actor in the state tree
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
