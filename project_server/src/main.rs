mod multi_threaded_server;
mod single_threaded_server;

fn main() {
    single_threaded_server::run();
    multi_threaded_server::run();
}
