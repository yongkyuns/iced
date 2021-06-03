#[cfg(not(feature = "dynamic_linking"))]
pub use iced::*;

#[cfg(feature = "dynamic_linking")]
pub use iced_dynamic_internal::*;
