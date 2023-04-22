fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    let result = stringify_name_with_title(&name);
    println!("{first}");
    println!("{result}");
}

/// [try 1] Does not compile because we can't modify the passed vector
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

/// [try 2] Functions should not mutate their inputs if the caller would not expect it
// fn stringify_name_with_title(name: &mut Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

/// [try 3] It is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String.
// fn stringify_name_with_title(mut name: Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

/// [try 4] Works, but by cloning name, we are allowed to mutate the local copy of the vector. However, the clone copies every string in the input
fn stringify_name_with_title_okay(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

/// [try 5] We can avoid unnecessary copies by adding the suffix later
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
