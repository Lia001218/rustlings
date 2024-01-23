// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



#[test]
fn main() {
    let mut x = 100;
    let mut y = x;
    let mut z = x;
    
    y += 100;
    z += 1000;
    println!("{} {} {}", x,y,z);
    x +=z;
    assert_eq!(x, 1200);
}
