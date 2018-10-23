impl From<Icon> for char {
    fn from(icon: Icon) -> char {
        icon_to_char(icon)
    }
}

/// Material icons font (`MaterialIcons-Regular.ttf`), licensed under the Apache 2.0 license
/// While attribution is not strictly required, it is appreciated.
/// See https://github.com/google/material-design-icons for more information
pub const FONT: &[u8] = include_bytes!("../assets/MaterialIcons-Regular.ttf");

use std::fmt;
impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", icon_to_char(*self))
    }
}
