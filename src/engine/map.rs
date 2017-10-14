//#[macro_use]
use common;

/// These are the X11 color names (from http://cng.seas.rochester.edu/CNG/docs/x11color.html).
/// UIs are expected to handle these colors the best they can (terminals for example often
/// don't support true color so some of these colors will map to the same screen color).
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
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

pub struct Square {
    pub symbol: char,
    pub back_color: Color,
    pub fore_color: Color,
}

pub fn build_map(text: &str) -> Vec<Vec<Square>> {
    let mut map = Vec::new();

    let it = text.chars();
    let it = it.skip_while(|c| c.is_whitespace());

    let mut row = Vec::new();
    let mut width = 0;
    for c in it {
        match c {
            '\n' => {
                fatal_error_if!(
                    width != 0 && width != row.len(),
                    "row {}'s width doesn't match the widths of the earlier rows", // TODO: include the map origin
                    map.len() + 1
                );
                width = row.len();
                map.push(row);
                row = Vec::new();
            }
            '#' => {
                row.push(Square {
                    symbol: ' ',
                    back_color: Color::SaddleBrown,
                    fore_color: Color::Black,
                })
            }
            '=' => {
                row.push(Square {
                    symbol: ' ',
                    back_color: Color::Bisque,
                    fore_color: Color::Black,
                })
            }
            'w' => {
                row.push(Square {
                    symbol: ' ',
                    back_color: Color::DodgerBlue,
                    fore_color: Color::Black,
                })
            }
            _ => {
                row.push(Square {
                    symbol: c,
                    back_color: Color::LightGrey,
                    fore_color: Color::Black,
                })
            }
        }
    }
    map.push(row);

    map
}
