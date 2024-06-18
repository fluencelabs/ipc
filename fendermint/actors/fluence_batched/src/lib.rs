use fvm_ipld_encoding::ipld_block::IpldBlock;
use fvm_shared::error::{ErrorNumber, ExitCode};
use fvm_shared::MethodNum;
use num_derive::FromPrimitive;

use fil_actors_runtime::runtime::{ActorCode, Runtime};
use fil_actors_runtime::{actor_dispatch, FIRST_EXPORTED_METHOD_NUMBER};
use fil_actors_runtime::{actor_error, ActorError};

use crate::types::RandomXResult;
use crate::types::{RandomXArguments, RandomXArgumentsBatched, RandomXResultBatched};

pub mod types;

pub const FLUENCE_BATCHED_ACTOR_NAME: &str = "fluence-batched";
const SYSCALL_FAILED_EXIT_CODE: u32 = 0x10000000;

#[cfg(feature = "fil-actor")]
fil_actors_runtime::wasm_trampoline!(FluenceActorBatched);

/// Account actor methods available
#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    RunRandomX = frc42_dispatch::method_hash!("RunRandomX"),
    RunRandomXBatched = frc42_dispatch::method_hash!("RunRandomXBatched"),
}

/// Account Actor
pub struct FluenceActorBatched;

impl FluenceActorBatched {
    /// Run RandomX with the provided parameters and returns its result hash.
    pub fn run_randomx(
        rt: &impl Runtime,
        params: RandomXArguments,
    ) -> Result<RandomXResult, ActorError> {
        log::info!("actor::run_randomx: start {params:?}");
        rt.validate_immediate_caller_accept_any()?;

        let result = fluence_actor_sdk::run_randomx(params.global_nonce, params.local_nonce)
            .map_err(randomx_failed)?;
        log::info!("actor::run_randomx: result is {result:?}");

        let result = RandomXResult { result };
        Ok(result)
    }

    /// Run RandomX with the provided parameters and returns its result hash.
    pub fn run_randomx_batched(
        rt: &impl Runtime,
        params: RandomXArgumentsBatched,
    ) -> Result<RandomXResultBatched, ActorError> {
        use fluence_actor_sdk::TARGET_HASH_SIZE;
        use fvm_ipld_encoding::BytesDe;

        log::info!("actor::run_randomx: start {params:?}");
        rt.validate_immediate_caller_accept_any()?;
        let result_len = params.global_nonce.len();

        // The result is a vector of hashes, each hash is TARGET_HASH_SIZE bytes.
        let result =
            fluence_actor_sdk::run_randomx_batched(&params.global_nonce, &params.local_nonce)
                .map_err(randomx_failed)?;

        log::info!("actor::run_randomx: result is {result:?}");
        log::info!("actor::run_randomx: result batch len is {result_len:?}");

        let result = result[..result_len * TARGET_HASH_SIZE]
            .chunks_exact(TARGET_HASH_SIZE)
            .map(|chunk| {
                let mut hash_arr = vec![0u8; TARGET_HASH_SIZE];
                hash_arr.copy_from_slice(chunk);
                BytesDe(hash_arr)
            })
            .collect();

        let result = RandomXResultBatched { result };
        Ok(result)
    }

    /// Fallback method for unimplemented method numbers.
    pub fn fallback(
        rt: &impl Runtime,
        method: MethodNum,
        _: Option<IpldBlock>,
    ) -> Result<Option<IpldBlock>, ActorError> {
        rt.validate_immediate_caller_accept_any()?;
        if method >= FIRST_EXPORTED_METHOD_NUMBER {
            Ok(None)
        } else {
            Err(actor_error!(unhandled_message; "invalid method: {}", method))
        }
    }
}

impl ActorCode for FluenceActorBatched {
    type Methods = Method;

    fn name() -> &'static str {
        FLUENCE_BATCHED_ACTOR_NAME
    }

    actor_dispatch! {
        RunRandomX => run_randomx,
        RunRandomXBatched => run_randomx_batched,
        _ => fallback,
    }
}

fn randomx_failed(error_num: ErrorNumber) -> ActorError {
    log::error!("actor::run_randomx: run_randomx failed with {error_num}");

    let err_msg = format!("run_randomx syscall failed with {error_num}");
    ActorError::checked(ExitCode::new(SYSCALL_FAILED_EXIT_CODE), err_msg, None)
}
