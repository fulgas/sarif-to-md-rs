#[allow(clippy::all)]
#[allow(dead_code)]
#[rustfmt::skip]
mod bindings {
    include!("snyk_container.rs");
}
pub use bindings::*;
