pub mod bit_reverse;
pub mod blake2s;
pub mod cm31;
pub mod column;
pub mod m31;
pub mod qm31;
mod utils;

#[derive(Copy, Clone, Debug)]
pub struct SimdBackend;