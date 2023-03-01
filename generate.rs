//! Generates the Rust enum that corresponds to the codepoint file
use std::fs::File;
use std::io::Write;
use std::char::from_u32;

const RUST_CODE_PREAMBLE: &str = include_str!("preamble.rs");
const RUST_CODE_START_1: &str = "
#[macro_use]
extern crate enum_iterator;

/// Icon containing all possible icon names as enum discriminants
#[repr(C)]
#[derive(Debug, Copy, Clone, IntoEnumIterator, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
pub fn icon_to_html_name(icon: &Icon) -> &'static str {
    use self::Icon::*;
    match *icon {
";
const RUST_CODE_END_4: &str = r#"    }
}
"#;

fn fix_icon_name(name: &str) -> &str {
    // Rust icons can't start with number
    match name {
        "10k" => "ten_k",
        "10mp" => "ten_mp",
        "11mp" => "eleven_mp",
        "123" => "one_two_three",
        "12mp" => "twelve_mp",
        "13mp" => "thirteen_mp",
        "14mp" => "fourteen_mp",
        "15mp" => "fifteen_mp",
        "16mp" => "sixteen_mp",
        "17mp" => "seventeen_mp",
        "18_up_rating" => "eighteen_up_rating",
        "18mp" => "eighteen_mp",
        "19mp" => "nineteen_mp",
        "1k" => "one_k",
        "1k_plus" => "one_k_plus",
        "1x_mobiledata" => "one_x_mobiledata",
        "20mp" => "twenty_mp",
        "21mp" => "twenty_one_mp",
        "22mp" => "twenty_two_mp",
        "23mp" => "twenty_three_mp",
        "24mp" => "twenty_four_mp",
        "2k" => "two_k",
        "2k_plus" => "two_k_plus",
        "2mp" => "two_mp",
        "30fps" => "thirty_fps",
        "30fps_select" => "thirty_fps_select",
        "360" => "three_sixty",
        "3d_rotation" => "rotation_3d",
        "3g_mobiledata" => "mobiledata_3g",
        "3k" => "three_k",
        "3k_plus" => "three_k_plus",
        "3mp" => "three_mp",
        "3p" => "three_p",
        "4g_mobiledata" => "mobiledata_4g",
        "4g_plus_mobiledata" => "mobiledata_4g_plus",
        "4k" => "four_k",
        "4k_plus" => "four_k_plus",
        "4mp" => "four_mp",
        "5g" => "five_g",
        "5k" => "five_k",
        "5k_plus" => "five_k_plus",
        "5mp" => "five_mp",
        "60fps" => "sixty_fps",
        "60fps_select" => "sixty_fps_select",
        "6_ft_apart" => "six_ft_apart",
        "6k" => "six_k",
        "6k_plus" => "six_k_plus",
        "6mp" => "six_mp",
        "7k" => "seven_k",
        "7k_plus" => "seven_k_plus",
        "7mp" => "seven_mp",
        "8k" => "eight_k",
        "8k_plus" => "eight_k_plus",
        "8mp" => "eight_mp",
        "9k" => "nine_k",
        "9k_plus" => "nine_k_plus",
        "9mp" => "nine_mp",
        name => name,
    }
}

fn main() {
    const CODEPOINTS: &str = include_str!("./assets/codepoints.txt");

    let codepoints = CODEPOINTS.lines().map(|line| {
        let mut whitespace_iterator = line.split_whitespace();
        let icon_name = fix_icon_name(whitespace_iterator.next().unwrap());

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
        name = fix_icon_name(name);

        let enum_str = format!("        {} => {:?},\n", icon_name, name);
        file.write(enum_str.as_bytes()).unwrap();
    }
    file.write(RUST_CODE_END_4.as_bytes()).unwrap();

}
