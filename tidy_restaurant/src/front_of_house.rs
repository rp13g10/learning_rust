// This structure, where a module has both a file and a directory is
// considered the standard way to define modules. However, older code may
// have the code here defined inside front_of_house/mod.rs instead. This is
// equivalent code, and still compiles without issue. The newer approach
// was adopted to avoid confusion when editing multiple files called mod.rs
pub mod hosting;

pub mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}