pub mod buffer;
pub mod data_buffer;
pub mod infomotor;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(
    feature = "use_serde",
    derive(serde::Deserialize, derive_more::Display),
    display(
        fmt = "time(ms): {time}, enc(pulse): {pulse_encoder}, rpm(pulse): {pulse_rpm}, temp(celcius): {temperature:.2}"
    )
)]
pub struct SerialData {
    pub time: u32, // in ms
    pub pulse_encoder: u32,
    pub pulse_rpm: u32,
    pub temperature: f32,
}

impl SerialData {
    pub const SIZE: usize = ::core::mem::size_of::<SerialData>();
    pub const DELIM: u8 = b'|';

    #[cfg(not(feature = "use_bincode"))]
    #[inline(always)]
    pub fn from_bytes(bytes: &'_ [u8]) -> Option<Self> {
        if bytes.len() != Self::SIZE {
            return None;
        }
        Some(unsafe { std::ptr::read::<Self>(bytes.as_ptr() as *const _) })
    }

    #[cfg(feature = "use_bincode")]
    #[inline(always)]
    pub fn from_bytes(bytes: &'_ [u8]) -> Option<Self> {
        bincode::deserialize(bytes).ok()
    }
}
