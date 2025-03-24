pub trait Slug {
    fn slug(&self) -> String;
}

impl Slug for String {
    /// Returns a slug for the string.
    ///
    /// The slug is created by:
    ///
    /// 1. Replacing 'Ü' with 'u'
    /// 2. Converting the string to lowercase
    /// 3. Replacing all non-alphanumeric characters with hyphens
    /// 4. Replacing multiple consecutive hyphens with a single hyphen
    /// 5. Trimming any trailing hyphens
    fn slug(&self) -> String {
        self.replace("Ü", "u")
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .replace(" ", "-")
            .trim_ascii()
            .replace("---", "-")
            .replace("--", "-")
            .trim_end_matches('-')
            .trim_start_matches('-')
            .to_string()
    }
}
