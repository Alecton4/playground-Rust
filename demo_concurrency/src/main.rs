mod demo_message_passing;
mod demo_state_sharing;
mod demo_thread_and_closure;

fn main() {
    demo_thread_and_closure::demo_spawn();
    demo_thread_and_closure::demo_join();
    demo_thread_and_closure::demo_join_2();
    demo_thread_and_closure::demo_move_error();
    demo_thread_and_closure::demo_move();

    demo_message_passing::demo_mpsc_basics();
    demo_message_passing::demo_receiver_waiting();
    demo_message_passing::demo_multi_producer();

    demo_state_sharing::demo_mutex_basics();
    demo_state_sharing::demo_multi_thread_error_1();
    demo_state_sharing::demo_multi_thread_error_2();
    demo_state_sharing::demo_multi_thread_with_Arc();
    demo_state_sharing::demo_deadlock();
}
