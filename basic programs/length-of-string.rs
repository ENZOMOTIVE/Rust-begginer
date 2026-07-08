// Takes string as an input and returns its length

fn main() {
    let name = String::from("Hello Aman");
    let length = get_string_length_chars(name);
    println!("The number of characters in the string is : {}", length);
}

fn get_string_length_chars(str: String) -> usize {
    str.chars().count()
}
