//extern crate extern_crate_0; // error[E0463]: can't find crate for `extern_crate_0`
pub mod my_mod_1;

#[cfg(test)]
mod tests {
//    extern crate my_crate; // error[E0463]: can't find crate for `my_crate`
//    extern crate super::my_crate; // error: expected identifier, found keyword `super`
//    use super::my_mod_1; // error[E0432]: unresolved import `super::my_mod_1`
//    use my_mod_1; // error[E0432]: unresolved import `my_mod_1`
//    mod my_mod_1; // error[E0583]: file not found for module `my_mod_1`
//    mod super::my_mod_1; // error: expected identifier, found keyword `super`
    use super::*;

    #[test]
    fn test_my_mod_1_public() {
//        assert_eq!(my_crate::my_mod_1::public(), String::from("my_mod_1::public()")); // error[E0433]: failed to resolve: use of undeclared type or module `my_crate`
        assert_eq!(my_mod_1::public(), String::from("my_mod_1::public()")); // error[E0433]: failed to resolve: use of undeclared type or module `my_crate`
    }
    #[test]
    fn test_my_mod_1_private() {
//        assert_eq!(my_crate::my_mod_1::private(), String::from("my_mod_1::private()")); // error[E0433]: failed to resolve: use of undeclared type or module `my_crate`
//        assert_eq!(my_mod_1::private(), String::from("my_mod_1::private()")); // error[E0603]: function `private` is private
    }
}
