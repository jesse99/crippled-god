pub type Hook = fn(&str) -> (); // this isn't a Fn because then we'd have to box it and we get: error[E0493] destructors in statics are an unstable feature

static mut HOOK: Option<Hook> = None;

pub fn set_fatal_hook(hook: Hook) {
    unsafe {
        HOOK = Some(hook);
    }
}

pub fn fatal_err(message: &str) -> ! {
    unsafe {
        if let Some(hook) = HOOK.as_ref() {
            hook(message);
        } else {
            println!("{}", message);
        }
    }
    panic!()
}

macro_rules! fatal_error
{
	() => (common::fatal_err(""));
	($msg:expr) => (common::fatal_err($msg));
	($fmt:expr, $($arg:tt)*) => (common::fatal_err(&format!($fmt, $($arg)*)));
}

macro_rules! fatal_error_if
{
	($predicate:expr) => (if $predicate {common::fatal_err("")});
	($predicate:expr, $msg:expr) => (if $predicate {common::fatal_err($msg)});
	($predicate:expr, $fmt:expr, $($arg:tt)*) => (if $predicate {common::fatal_err(&format!($fmt, $($arg)*))});
}
