// Test that cfg_attr with multiple attributes actually emits both attributes.
// This is done by emitting two attributes that cause new warnings, and then
// triggering those warnings.

// compile-pass

#![warn(unused_must_use)]
#![feature(cfg_attr_multi)]

#[cfg_attr(all(), deprecated, must_use)]
struct MustUseDeprecated {}

impl MustUseDeprecated { //~ warning: use of deprecated item
    fn new() -> MustUseDeprecated { //~ warning: use of deprecated item
        MustUseDeprecated {} //~ warning: use of deprecated item
    }
}

fn main() {
    MustUseDeprecated::new(); //~ warning: use of deprecated item
    //| warning: unused `MustUseDeprecated` which must be used
}
