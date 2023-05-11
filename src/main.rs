mod app;
use app::App;

fn main() {
    let app = App::default();

    app.mode_select();
}
