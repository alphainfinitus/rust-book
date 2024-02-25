fn main() {
    let mut str = String::from("hello world");

    let length = calculate_length(&str);

    println!("The length of '{}' is {}.", str, length);

    change(&mut str);

    println!("The new string  is \"{}\"", str);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", from India <3");
}
