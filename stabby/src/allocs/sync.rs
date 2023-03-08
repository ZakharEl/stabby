use crate::{self as stabby, abi::IntoDyn};
use alloc::sync::*;

impl stabby::abi::IPtr for Arc<()> {
    unsafe fn as_ref<U>(&self) -> &U {
        let this: &() = self;
        core::mem::transmute(this)
    }
}
impl stabby::abi::IPtrOwned for Arc<()> {
    fn drop(this: &mut core::mem::ManuallyDrop<Self>, drop: unsafe extern "C" fn(&mut ())) {
        // Increment the weak count to guarantee the allocation won't be freed
        let weak = Arc::downgrade(this);
        unsafe {
            // If this is the last strong
            if let Some(inner) = Arc::get_mut(this) {
                // Hold onto the pointer to the target
                let inner = core::mem::transmute(inner);
                // Drop the Arc ASAP to avoid other Weaks getting upgraded
                core::mem::ManuallyDrop::drop(this);
                // Drop the content, `weak` needs to live at least up to here for the allocation to be guaranteed
                (drop)(inner);
            } else {
                // Otherwise just decrement the strong count
                core::mem::ManuallyDrop::drop(this);
            }
        }
        core::mem::drop(weak);
    }
}

impl<T: Sized> IntoDyn for Arc<T> {
    type Anonymized = Arc<()>;
    type Target = T;
    fn anonimize(self) -> Self::Anonymized {
        unsafe { core::mem::transmute(self) }
    }
}