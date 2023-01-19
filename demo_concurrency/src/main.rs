mod demo_message_passing;
mod demo_thread_and_closure;

fn main() {
    demo_thread_and_closure::demo_spawn();
    demo_thread_and_closure::demo_join();
    demo_thread_and_closure::demo_join_2();
    demo_thread_and_closure::demo_move_error();
    demo_thread_and_closure::demo_move();
}
