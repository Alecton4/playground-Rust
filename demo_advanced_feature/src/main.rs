mod demo_advanced_trait;
mod demo_unsafe;

fn main() {
    demo_unsafe::demo_raw_pointer::demo_raw_pointer_from_ref();
    demo_unsafe::demo_raw_pointer::demo_raw_pointer_to_arbitrary_mem();
    demo_unsafe::demo_raw_pointer::demo_deref_raw_pointer();

    demo_unsafe::demo_unsafe_func_method::demo_unsafe_call();
    demo_unsafe::demo_unsafe_func_method::demo_safe_abstraction_over_unsafe();

    use demo_unsafe::demo_static_vars::HELLO_WORLD;
    println!("name is: {}", HELLO_WORLD);
    demo_unsafe::demo_static_vars::add_to_count(8964);
    demo_unsafe::demo_static_vars::print_count();
}
