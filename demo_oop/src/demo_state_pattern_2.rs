// REF: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html#trade-offs-of-the-state-pattern

use demo_oop::lib_demo_state_pattern_2::Post;

pub fn demo_use_rust_strength() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
