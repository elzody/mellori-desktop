fn main() {
  glib_build_tools::compile_resources(&["src/res/"], "src/res/res.gresource.xml", "res.gresource");
}
