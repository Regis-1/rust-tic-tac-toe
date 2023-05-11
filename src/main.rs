mod app;

use app::App;

fn main() {
    let app = App::default();

    app.show_menu();
    let selected_mode = app.mode_select().unwrap_or_else(|err| {
        eprintln!("ERR: Error while mode selection -> {err}");
        std::process::exit(-1);
    });

    app.run_game(selected_mode);
}
