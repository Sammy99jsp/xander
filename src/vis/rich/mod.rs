//! Rich formatting.

pub trait RichFormatting: std::fmt::Display {
    fn text(&self) -> String {
        <Self as std::string::ToString>::to_string(self)
    }

    fn html(&self) -> String;
}
