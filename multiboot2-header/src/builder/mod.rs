//! Module for the builder-feature.

mod header;
mod information_request;
pub(self) mod traits;

pub use header::Multiboot2HeaderBuilder;
pub use information_request::InformationRequestHeaderTagBuilder;
