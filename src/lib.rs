use std::sync::Arc;

mod command;
mod device;

pub use self::command::*;
pub use self::device::*;

type Global = core::hub::Global<core::hub::IdentityManagerFactory>;

lazy_static::lazy_static! {
    static ref GLOBAL: Arc<Global> = Arc::new(Global::new("wgpu", core::hub::IdentityManagerFactory));
}
