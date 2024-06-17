use fluence_actor_sdk::TARGET_HASH_SIZE;
use fvm_ipld_encoding::strict_bytes;
use fvm_ipld_encoding::tuple::*;
use fvm_ipld_encoding::BytesDe;

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RandomXArguments {
    #[serde(with = "strict_bytes")]
    pub global_nonce: Vec<u8>,
    #[serde(with = "strict_bytes")]
    pub local_nonce: Vec<u8>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RandomXResult {
    #[serde(with = "strict_bytes")]
    pub result: [u8; TARGET_HASH_SIZE],
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RandomXArgumentsBatched {
    // To be replaced with Vec<Nonce> once we have a proper type in shared for it
    pub global_nonce: Vec<BytesDe>,
    pub local_nonce: Vec<BytesDe>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RandomXResultBatched {
    pub result: Vec<BytesDe>,
}
