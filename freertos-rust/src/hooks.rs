use crate::base::*;
use crate::prelude::v1::String;
use crate::utils::*;

type Callback = fn();

pub struct FreeRtosHooks {
    on_assert: Callback,
}

impl FreeRtosHooks {
    pub fn set_on_assert(&mut self, c: Callback) {
        self.on_assert = c;
    }

    fn do_on_assert(&self) {
        (self.on_assert)();
    }
}

// TODO: It's unsafe to use, we should build some safe wrapper around
pub static mut FREERTOS_HOOKS: FreeRtosHooks = FreeRtosHooks { on_assert: || {} };

#[allow(unused_doc_comments)]
#[no_mangle]
pub extern "C" fn vAssertCalled(file_name_ptr: FreeRtosCharPtr, line: FreeRtosUBaseType) {
}
