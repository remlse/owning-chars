mod owning_chars;
use owning_chars::OwningChars;

pub fn hello_world_chars() -> impl Iterator<Item = char> {
    // Does not work! Regular .chars() is tied to the lifetime of the string.
    // return String::from("Hello, world!").chars();

    String::from("Hello, world!").owning_chars()
}
