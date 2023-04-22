fn main() {
    // cannot borrow `a[_]` as immutable because it is also borrowed as mutable
    // let mut a = [0, 1, 2, 3];
    // let x = &mut a[0];
    // let y = &a[1];
    // *x += *y;

    // Rust often provides a function in the standard library that can work around the borrow checker
    let mut a = [0, 1, 2, 3];
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x += *y;

    // we could use an unsafe block to accomplish
    let mut a2 = [0, 1, 2, 3];
    let x2 = &mut a2[0] as *mut i32;
    let y2 = &a2[1] as *const i32;
    unsafe {
        *x2 += *y2;
    } // DO NOT DO THIS unless you know what you're doing!
}
