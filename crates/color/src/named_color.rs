use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;

/// The semantic name of a [`crate::Color`] slot inside a
/// [`crate::Palette`].
///
/// The name vocabulary (`"light"`, `"dark"`, `"ink"`, `"accent"`,
/// `"paper"`, ...) is defined by the consuming crate - typically as
/// constants or an enum converting into `NamedColor`. This type is
/// only the technical key: hashable, comparable and serializable.
///
/// `NamedColor::new` is `const`, so name registries can be defined as
/// associated constants; owned names are supported via `From<String>`.
///
/// ```
/// use smearor_commons_color::NamedColor;
///
/// const LIGHT: NamedColor = NamedColor::new("light");
/// assert_eq!(LIGHT.as_str(), "light");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedColor(Cow<'static, str>);

impl NamedColor {
    /// Creates a color name from a `&'static str` without allocating.
    pub const fn new(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }

    /// The name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Static names do not allocate.
impl From<&'static str> for NamedColor {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}

/// Dynamic names are stored owned.
impl From<String> for NamedColor {
    fn from(name: String) -> Self {
        Self(Cow::Owned(name))
    }
}

impl AsRef<str> for NamedColor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Enables `HashMap` lookups by `&str` (`Hash`/`Eq` match `str`).
impl Borrow<str> for NamedColor {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Display for NamedColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The wire format is the plain name string.
impl Serialize for NamedColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Deserialized names are always owned.
impl<'de> Deserialize<'de> for NamedColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(Cow::Owned(String::deserialize(deserializer)?)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIGHT: NamedColor = NamedColor::new("light");

    #[test]
    fn const_new_borrows_the_name() {
        assert_eq!(LIGHT.as_str(), "light");
    }

    #[test]
    fn borrowed_and_owned_names_are_equal() {
        let owned = NamedColor::from(String::from("light"));
        assert_eq!(LIGHT, owned);
        assert_eq!(LIGHT.as_str(), owned.as_str());
    }

    #[test]
    fn borrowed_and_owned_names_hash_equal() {
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(LIGHT.clone(), 42);
        assert_eq!(scores.get("light"), Some(&42));
        assert_eq!(scores.get(&NamedColor::from(String::from("light"))), Some(&42));
    }

    #[test]
    fn display_prints_the_name() {
        assert_eq!(LIGHT.to_string(), "light");
    }

    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&LIGHT).unwrap();
        assert_eq!(json, "\"light\"");
        let parsed: NamedColor = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, LIGHT);
    }
}
