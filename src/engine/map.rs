/// These are the X11 color names (from http://cng.seas.rochester.edu/CNG/docs/x11color.html).
/// UIs are expected to handle these colors the best they can (terminals for example often
/// don't support true color so some of these colors will map to the same screen color).
pub enum Color {
    LightPink,
    Pink,
    Crimson,
    LavenderBlush,
    PaleVioletRed,
    HotPink,
    DeepPink,
    MediumVioletRed,
    Orchid,
    Thistle,
    Plum,
    Violet,
    Magenta,
    Fuchsia,
    DarkMagenta,
    Purple,
    MediumOrchid,
    DarkViolet,
    DarkOrchid,
    Indigo,
    BlueViolet,
    MediumPurple,
    MediumSlateBlue,
    SlateBlue,
    DarkSlateBlue,
    Lavender,
    GhostWhite,
    Blue,
    MediumBlue,
    MidnightBlue,
    DarkBlue,
    Navy,
    RoyalBlue,
    CornflowerBlue,
    LightSteelBlue,
    LightSlateGray,
    SlateGray,
    DodgerBlue,
    AliceBlue,
    SteelBlue,
    LightSkyBlue,
    SkyBlue,
    DeepSkyBlue,
    LightBlue,
    PowderBlue,
    CadetBlue,
    Azure,
    LightCyan,
    PaleTurquoise,
    Cyan,
    Aqua,
    DarkTurquoise,
    DarkSlateGray,
    DarkCyan,
    Teal,
    MediumTurquoise,
    LightSeaGreen,
    Turquoise,
    Aquamarine,
    MediumAquamarine,
    MediumSpringGreen,
    MintCream,
    SpringGreen,
    MediumSeaGreen,
    SeaGreen,
    Honeydew,
    LightGreen,
    PaleGreen,
    DarkSeaGreen,
    LimeGreen,
    Lime,
    ForestGreen,
    Green,
    DarkGreen,
    Chartreuse,
    LawnGreen,
    GreenYellow,
    DarkOliveGreen,
    YellowGreen,
    OliveDrab,
    Beige,
    LightGoldenrodYellow,
    Ivory,
    LightYellow,
    Yellow,
    Olive,
    DarkKhaki,
    LemonChiffon,
    PaleGoldenrod,
    Khaki,
    Gold,
    Cornsilk,
    Goldenrod,
    DarkGoldenrod,
    FloralWhite,
    OldLace,
    Wheat,
    Moccasin,
    Orange,
    PapayaWhip,
    BlanchedAlmond,
    NavajoWhite,
    AntiqueWhite,
    Tan,
    BurlyWood,
    Bisque,
    DarkOrange,
    Linen,
    Peru,
    PeachPuff,
    SandyBrown,
    Chocolate,
    SaddleBrown,
    Seashell,
    Sienna,
    LightSalmon,
    Coral,
    OrangeRed,
    DarkSalmon,
    Tomato,
    MistyRose,
    Salmon,
    Snow,
    LightCoral,
    RosyBrown,
    IndianRed,
    Red,
    Brown,
    FireBrick,
    DarkRed,
    Maroon,
    White,
    WhiteSmoke,
    Gainsboro,
    LightGrey,
    Silver,
    DarkGray,
    Gray,
    DimGray,
    Black,
}


// pub struct Square {
//     symbol: char,
//     back_color: Color,
//     fore_color: Color,
// }

// fn fatal_err(message: &str) -> ! {
//     let mut stdout = std::io::stdout().into_raw_mode().unwrap();
//     let _ = write!(stdout, "{}\n", message);
//     let _ = write!(
//         stdout,
//         "{}{}\n",
//         termion::cursor::Restore,
//         termion::cursor::Show
//     );
//     panic!();
// }

// macro_rules! fatal_error_if
// {
// 	($predicate:expr) => (if $predicate {fatal_err("")});
// 	($predicate:expr, $msg:expr) => (if $predicate {fatal_err($msg)});
// 	($predicate:expr, $fmt:expr, $($arg:tt)*) => (if $predicate {fatal_err(&format!($fmt, $($arg)*))});
// }

// fn build_map(text: &str) -> Vec<Vec<Square>> {
//     let mut map = Vec::new();

//     let it = text.chars();
//     let it = it.skip_while(|c| c.is_whitespace());

//     let mut row = Vec::new();
//     let mut width = 0;
//     for c in it {
//         match c {
//             '\n' => {
//                 fatal_error_if!(
//                     width != 0 && width != row.len(),
//                     "row {}'s width doesn't match the widths of the earlier rows", // TODO: include the map origin
//                     map.len() + 1
//                 );
//                 width = row.len();
//                 map.push(row);
//                 row = Vec::new();
//             }
//             '#' => {
//                 row.push(Square {
//                     symbol: ' ',
//                     back_color: termion::color::AnsiValue::rgb(1, 0, 0),
//                     fore_color: termion::color::AnsiValue::grayscale(0),
//                 })
//             }
//             '=' => {
//                 row.push(Square {
//                     symbol: ' ',
//                     back_color: termion::color::AnsiValue::rgb(3, 1, 0),
//                     fore_color: termion::color::AnsiValue::grayscale(0),
//                 })
//             }
//             'w' => {
//                 row.push(Square {
//                     symbol: ' ',
//                     back_color: termion::color::AnsiValue::rgb(0, 0, 4),
//                     fore_color: termion::color::AnsiValue::grayscale(0),
//                 })
//             }
//             _ => {
//                 row.push(Square {
//                     symbol: c,
//                     back_color: termion::color::AnsiValue::grayscale(0),
//                     fore_color: termion::color::AnsiValue::grayscale(0),
//                 })
//             }
//         }
//     }
//     map.push(row);

//     map
// }
