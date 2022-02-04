#[cfg(not(target_arch = "wasm32"))]
pub mod http;
#[cfg(target_arch = "wasm32")]
pub mod resize;
