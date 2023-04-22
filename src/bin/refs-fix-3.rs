fn main() {
    let src = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
    ];
    let mut dst = vec![String::from("two")];
    println!("dst is now {:?}", dst);
    add_big_strings_try3(&mut dst, &src);
    println!("dst is now {:?}", dst);
}

/// clone `largest`
/// may cause a performance hit for allocating and copying the string data
fn add_big_strings_try1(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

/// perform all the length comparisons first, and then mutate dst afterwards
/// this also causes a performance hit for allocating the vector to_add
fn add_big_strings_try2(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest.len())
        .cloned()
        .collect();
    dst.extend(to_add);
}

/// copy out the length, since we don't need the contents of largest, just its length
/// most idiomatic and the most performant
fn add_big_strings_try3(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
