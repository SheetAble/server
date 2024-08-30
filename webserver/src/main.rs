mod database;
mod server;

fn main() {
    println!("Starting server...");
    server::run_server();
}
