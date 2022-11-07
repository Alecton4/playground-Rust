pub mod hosting; // content of the child module is place in a file with the same name under a folder with the same name with the parent module

pub mod serving {
    fn take_order() {}
    pub fn serve_order() {}
    fn take_payment() {}
}
