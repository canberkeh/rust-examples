fn main() {
    let s1 : String = String::from("Hello ");
    let s2 : String = String::from("World!");
    let concatenated_string = concatenate_strings(&s1, &s2);
    println!("Concatenated string is {}", concatenated_string);
}

fn concatenate_strings(first_str: &str, second_str: &str) -> String {
    let mut result : String = String::from("");
    result.push_str(first_str);
    result.push_str(second_str);
    result
 }