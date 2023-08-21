mod app;
mod tabs;
mod server;

use app::App;

fn main() {
    App::run().unwrap();
}
