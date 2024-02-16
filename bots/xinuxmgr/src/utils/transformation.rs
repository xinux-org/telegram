pub trait Capitalize {
    fn capitalize(&self) -> Self;
}

impl Capitalize for String {
    fn capitalize(&self) -> Self {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
