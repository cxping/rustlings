// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); //所有权被移交

    string_uppercase(data);
}

// Should not take ownership //不应该获取到所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership//应该获取所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
