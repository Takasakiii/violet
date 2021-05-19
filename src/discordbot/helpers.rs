pub struct SmallerString(String);

impl From<&String> for SmallerString {
    fn from(el: &String) -> Self {
        if el.len() > 1018 {
            SmallerString(format!("{}...", &el[..1015]))
        } else {
            SmallerString(el.clone())
        }
    }
}

impl From<SmallerString> for String {
    fn from(el: SmallerString) -> Self {
        el.0
    }
}
