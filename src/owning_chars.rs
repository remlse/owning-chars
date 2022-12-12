pub trait OwningChars {
    type OwnChars;
    fn owning_chars(self) -> Self::OwnChars;
}

#[allow(dead_code)]
pub struct StringOwnChars {
    // actually only has the lifetime of `str`
    // declared before `str` so it's dropped first
    iter: std::str::Chars<'static>,
    // safety: we must never move out of this String as long as iter is alive
    // (In the article, this is boxed. But String is already heap allocated,
    // so (I think) we don't need to box a second time.)
    // running `cargo +nightly miri run` gives no errors.
    str: String,
}

impl OwningChars for String {
    type OwnChars = StringOwnChars;

    fn owning_chars(self) -> Self::OwnChars {
        let iter = self.chars();
        let iter = unsafe { std::mem::transmute(iter) };
        StringOwnChars { iter, str: self }
    }
}

impl Iterator for StringOwnChars {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
