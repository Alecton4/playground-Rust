// REF: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
// REF: https://www.youtube.com/watch?v=VFmPwvhubow

use demo_oop::lib_demo_state_pattern::Post;

pub fn demo_state_pattern() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
