use core::fmt::Debug;
use core::hint::unreachable_unchecked;

#[derive(Debug)]
pub enum SlotMapError {
    OutOfCapacity,
}

/// Internal stable replacement for !.
#[derive(Debug)]
pub enum Never {}
impl From<SlotMapError> for Never {
    fn from(_: SlotMapError) -> Self {
        unreachable!()
    }
}

/// Returns if a is an older version than b, taking into account wrapping of
/// versions.
pub fn is_older_version(a: u16, b: u16) -> bool {
    let diff = a.wrapping_sub(b);
    diff >= (1 << 15)
}

/// An unwrapper that checks on debug, doesn't check on release.
/// UB if unwrapped on release mode when unwrap would panic.
pub trait UnwrapUnchecked<T> {
    // Extra underscore because unwrap_unchecked is planned to be added to the stdlib.
    unsafe fn unwrap_unchecked_(self) -> T;
}

impl<T> UnwrapUnchecked<T> for Option<T> {
    unsafe fn unwrap_unchecked_(self) -> T {
        if cfg!(debug_assertions) {
            self.unwrap()
        } else {
            match self {
                Some(x) => x,
                None => unreachable_unchecked(),
            }
        }
    }
}

impl<T, E: Debug> UnwrapUnchecked<T> for Result<T, E> {
    unsafe fn unwrap_unchecked_(self) -> T {
        if cfg!(debug_assertions) {
            self.unwrap()
        } else {
            match self {
                Ok(x) => x,
                Err(_) => unreachable_unchecked(),
            }
        }
    }
}
