use gtk::prelude::*;

pub struct App {
  window: gtk::Window,
}

impl App {
  pub fn new() -> Result<Self, String> {
    if gtk::init().is_err() {
      return Err(String::from("Failed to initialize GTK"));
    }

    if gio::resources_register_include!("res.gresource").is_err() {
      return Err(String::from("Unable to register GResources"));
    }

    // let window = gtk::Window::new(gtk::WindowType::Toplevel);
    // let button = gtk::Button::with_label("Some kind of button");

    //window.add(&button);

    let builder = gtk::Builder::from_resource("/org/mellori/Desktop/login_ui");
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    builder.expose_object("window", &window);

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
