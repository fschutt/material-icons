//! Generates the Rust enum that corresponds to the codepoint file
use std::fs::File;
use std::io::Write;
use std::char::from_u32;

const RUST_CODE_PREAMBLE: &str = include_str!("preamble.rs");
const RUST_CODE_START_1: &str = "
/// Icon containing all possible icon names as enum discriminants
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Icon {\n";
const RUST_CODE_END_1: &str = "}\n\n";
const RUST_CODE_START_2: &str = "
/// Converts an `Icon` to a `char`. Same as `format!(\"{}\", icon)` or `icon.to_char()`.
#[no_mangle]
pub fn icon_to_char(icon: Icon) -> char {
    use self::Icon::*;
    match icon {
";
const RUST_CODE_END_2: &str = "    }\n}\n";
const RUST_CODE_3: &str = include_str!("./extra.rs");
const RUST_CODE_START_4 : &str = "
/// Get icon HTML name
pub fn icon_to_html_name(icon: &Icon) -> String {
    use self::Icon::*;
    match *icon {
";
const RUST_CODE_END_4: &str = r#"    }.to_string()
}"#;


fn main() {
    const CODEPOINTS: &str = include_str!("./assets/codepoints.txt");

    let codepoints = CODEPOINTS.lines().map(|line| {
        let mut whitespace_iterator = line.split_whitespace();
        let mut icon_name = whitespace_iterator.next().unwrap(); // 3d_rotation

        // Rust icons can't start with number
        if icon_name == "3d_rotation" {
            icon_name = "rotation_3d";
        }



        // Adjust icon name to be PascalCase instead of snake_case
        let mut new_name = String::new();
        for icon_name_component in icon_name.split("_") {
            let first_char = icon_name_component.get(..1).unwrap();
            let rest = icon_name_component.get(1..).unwrap();
            new_name.extend(first_char.to_uppercase().chars());
            new_name.push_str(rest);
        }

        // Parse and convert codepoint
        let icon_codepoint = whitespace_iterator.next().unwrap(); // e84d => \u{e84d}
        let icon_usize = u32::from_str_radix(icon_codepoint, 16).unwrap();
        let icon_char = from_u32(icon_usize).unwrap();

        (new_name, icon_char, icon_name)
    }).collect::<Vec<(String, char, &str)>>();

    let mut file = File::create("./src/lib.rs").unwrap();

    file.write(RUST_CODE_PREAMBLE.as_bytes()).unwrap();

    // -- part 1: create the enum
    file.write(RUST_CODE_START_1.as_bytes()).unwrap();
    for (icon_name, _, _) in &codepoints {
        let enum_str = format!("    {},\n", icon_name);
        file.write(enum_str.as_bytes()).unwrap();
    }
    file.write(RUST_CODE_END_1.as_bytes()).unwrap();

    // -- part 2: match on the enum
    file.write(RUST_CODE_START_2.as_bytes()).unwrap();
    for (icon_name, icon_char, _) in &codepoints {
        let enum_str = format!("        {} => {:?},\n", icon_name, icon_char);
        file.write(enum_str.as_bytes()).unwrap();
    }
    file.write(RUST_CODE_END_2.as_bytes()).unwrap();

    // -- part 3: convenience code
    file.write(RUST_CODE_3.as_bytes()).unwrap();

    // -- part 4: match on the enum
    file.write(RUST_CODE_START_4.as_bytes()).unwrap();
    for (icon_name, _, mut name) in &codepoints {
        if icon_name == "Rotation3d" {
            name = "3d_rotation";
        }
        let enum_str = format!("        {} => {:?},\n", icon_name, name);
        file.write(enum_str.as_bytes()).unwrap();
    }
    file.write(RUST_CODE_END_4.as_bytes()).unwrap();

}
