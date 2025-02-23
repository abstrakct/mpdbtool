pub trait Slug {
    fn slug(&self) -> String;
}

impl Slug for String {
    /// Returns a slug for the string.
    ///
    /// The slug is created by
    ///
    /// 1. Replacing some special characters with a-z
    /// 2. Lowercasing the string
    /// 3. Replacing all non-alphanumeric characters with a hyphen
    /// 4. Replacing all spaces with a hyphen
    /// 5. Trimming any leading or trailing hyphens
    fn slug(&self) -> String {
        self.replace("Ü", "u")
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .replace(" ", "-")
            .trim_ascii()
            .to_string()
    }
}
