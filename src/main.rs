mod app;

use app::App;

fn main() {
    let mut app = App::default();
    app.run().unwrap_or_else(|err| {
        eprintln!("ERR: There was an error with an application -> {err}");
    });
}
