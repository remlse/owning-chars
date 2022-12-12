pub trait OwningChars {
    type OwnChar;
    fn owning_chars(self) -> Self::OwnChar;
}

pub struct StringOwnChar {}

impl OwningChars for String {
    type OwnChar = StringOwnChar;

    fn owning_chars(self) -> Self::OwnChar {
        todo!()
    }
}

impl Iterator for StringOwnChar {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub fn hello_world_chars() -> impl Iterator<Item = char> {
    // Does not work! Regular .chars() is tied to the lifetime of the string.
    // return String::from("Hello, world!").chars()

    String::from("Hello, world!").owning_chars()
}
