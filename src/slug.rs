pub trait Slug {
    fn slug(&self) -> String;
}

impl Slug for String {
    /// Converts the string to lowercase, replaces all non-ASCII characters with
    /// hyphens, replaces all spaces with hyphens, and trims leading and trailing
    /// whitespace. Returns the modified string.
    fn slug(&self) -> String {
        // convert to lowercase
        // replace all non-ascii characters with a hyphen
        // replace all spaces with a hyphen
        // trim leading and trailing whitespace
        self.to_lowercase()
            .replace(|c: char| !c.is_ascii(), "-")
            .replace(" ", "-")
            .trim_ascii()
            .to_string()
    }
}
