use std::process::exit;

mod app;
use app::App;

fn main() {
  let app = App::new();

  if let Ok(app) = app {
    println!("App created!");

    println!("Launching app");
    app.launch();
  } else {
    println!("App not created, something went wrong");
    exit(1);
  }
}
