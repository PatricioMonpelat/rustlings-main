// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

// I AM NOT DONE

fn main() {
    let data  = "Rust is great!".to_string();
    
    string_uppercase(&data);

    get_char(data);
    

}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    // data = &data.to_uppercase();
    let data2 = &data.to_uppercase();
    // let _last: Option<String> = None;
    println!("{}", data2);
}
