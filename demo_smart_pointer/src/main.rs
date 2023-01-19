// REF: https://doc.rust-lang.org/book/ch15-01-box.html

mod demo_box;
mod demo_refcell;
mod demo_reference_counting;
mod demo_trait_deref;
mod demo_trait_drop;
mod demo_reference_cycle;

fn main() {
    demo_box::demo_box_basics();
    demo_box::demo_list_using_box();
    demo_trait_deref::demo_deref();
    demo_trait_deref::demo_implicit_deref_coercion();
    demo_trait_drop::demo_drop_basics();
    demo_trait_drop::demo_early_drop();
    demo_reference_counting::demo_rc();
    demo_reference_counting::demo_rc_2();
    demo_refcell::demo_combine_rc_and_refcell();
    demo_reference_cycle::demo_cycle();
}
