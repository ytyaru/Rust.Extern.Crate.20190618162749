//extern crate my_crate; // error[E0463]: can't find crate for `my_crate`
extern crate extern_crate_0;
//mod extern_crate_0::my_mod_1; // error: expected one of `;` or `{`, found `::`
//mod my_mod_1;

fn main() {
    assert_eq!(extern_crate_0::my_mod_1::public(), String::from("my_mod_1::public()")); // error[E0433]: failed to resolve: use of undeclared type or module `my_crate`
//    assert_eq!(extern_crate_0::my_mod_1::private(), String::from("my_mod_1::private()")); // error[E0603]: function `private` is private

//    assert_eq!(my_crate::my_mod_1::public(), String::from("my_mod_1::public()")); // error[E0433]: failed to resolve: use of undeclared type or module `my_crate`
//    assert_eq!(my_crate::my_mod_1::private(), String::from("my_mod_1::private()")); // error[E0603]: function `private` is private

//    assert_eq!(my_mod_1::public(), String::from("my_mod_1::public()"));
//    assert_eq!(my_mod_1::private(), String::from("my_mod_1::private()")); // error[E0603]: function `private` is private
}

