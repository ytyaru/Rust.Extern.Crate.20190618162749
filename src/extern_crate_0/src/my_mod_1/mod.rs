//extern crate extern_crate_0; // error[E0463]: can't find crate for `extern_crate_0`

pub fn public() -> String { String::from("my_mod_1::public()") }
fn private() -> String { String::from("my_mod_1::private()") }

