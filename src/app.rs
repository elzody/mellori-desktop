use gtk::prelude::*;

pub struct App {
  window: gtk::Window,
}

impl App {
  pub fn new() -> Result<Self, String> {
    if gtk::init().is_err() {
      return Err(String::from("Failed to initialize GTK"));
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let button = gtk::Button::with_label("Some kind of button");

    window.add(&button);

    Ok(Self { window })
  }

  pub fn launch(&self) {
    self.window.show_all();
    self.run_event_loop();
  }

  fn run_event_loop(&self) {
    gtk::main();
  }
}
