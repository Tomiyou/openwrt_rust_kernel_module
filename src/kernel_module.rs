use crate::kernel_result::*;

pub trait KernelModule : Sized + Sync {
    fn init() -> KernelResult<Self>;
}
