// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

// Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
fn main() {
    let mut x = 100;
    let y = &mut x; //只能有一个可变的变量
    // let z = &mut x;
    *y += 100;
    *y += 1000;
    assert_eq!(x, 1200);
}
