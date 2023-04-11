fn main() {
  let first = String::from("Ferris");
  // name parameter takes ownership of the value
  // let full = add_suffix(first);
  // we must clone the variable we are moving if we want to use it later
  let full = add_suffix(first.clone());
  // This will fail without cloning as Strings ownership will be moved so it can't be borrowed later
  println!("{full}, originally {first}"); // first is now used here
}

fn add_suffix(mut name: String) -> String {
  name.push_str(" Jr.");
  name
}
