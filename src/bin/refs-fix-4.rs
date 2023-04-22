fn main() {
    // cannot move out of `*s_ref` which is behind a shared reference
    // let v: Vec<String> = vec![String::from("Hello world")];
    // let s_ref: &String = &v[0];
    // let s: String = *s_ref;

    // avoid taking ownership of the string and just use an immutable reference
    let v1: Vec<String> = vec![String::from("Hello world")];
    let s_ref1: &String = &v1[0];
    println!("{s_ref1}");

    // clone the data if you want to get ownership of the string while leaving the vector alone
    let v2: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v2[0].clone();
    s.push('!');
    println!("{s}");

    // you can use a method like Vec::remove to move the string out of the vector
    let mut v3: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v3.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v3.len() == 0);
}
