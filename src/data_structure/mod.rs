pub mod buffer;
pub mod data_buffer;
pub mod filter;
pub mod infomotor;
pub use filter::ExponentialFilter;

pub mod prelude {
    pub use super::buffer::*;
    pub use super::data_buffer::*;
    pub use super::infomotor::*;
    pub use super::SerialData;
}

// macro `repr(C)` agar dapat merepresentasikan struct ini sebagai
// struct yang sama pada bahasa pemrograman C
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, serde::Deserialize, derive_more::Display)]
#[display(fmt = r"SerialData {{ 
    period: {period},
    enc_max: {pulse_enc_max},
    enc_z: {pulse_enc_z},
    enc: {pulse_enc},
    rpm: {pulse_rpm},
    temp: {temperature}
}}")]
pub struct SerialData {
    pub period: u32,
    pub pulse_enc_max: u32,
    pub pulse_enc: u32,
    pub pulse_enc_z: u32,
    pub pulse_rpm: u32,
    pub temperature: f32,
}

impl SerialData {
    pub const SIZE: usize = ::core::mem::size_of::<SerialData>();
    pub const DELIM: u8 = b'\n';

    #[inline(always)]
    // konversi bytes (array of byte) secara menyamakan memory layout
    // memvalidasi apakah ukuran bytes sama dengan ukuran  data struct
    pub fn from_bytes(bytes: &'_ [u8]) -> Option<Self> {
        if bytes.len() != Self::SIZE {
            return None;
        }
        Some(unsafe { std::ptr::read::<Self>(bytes.as_ptr() as *const _) })
    }
}
