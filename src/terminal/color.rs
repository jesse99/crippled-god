use termion;

// These are the X11 color names (from http://cng.seas.rochester.edu/CNG/docs/x11color.html).
// In general we work with the X11 colors instead of AnsiColors because the X11 colors are
// a lot nicer to deal with.
#[allow(dead_code)]
#[derive(Clone, Copy)]
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

// See https://camo.githubusercontent.com/18622d6a234413cbc0aba27a09146797bf1eef4d/68747470733a2f2f692e696d6775722e636f6d2f4b696c72306d432e706e673f31
// and http://cng.seas.rochester.edu/CNG/docs/x11color.html
pub fn to_termion(color: Color) -> termion::color::AnsiValue {
	match color {
		Color::LightPink => termion::color::AnsiValue::rgb(5, 3, 5),
		Color::Pink => termion::color::AnsiValue::rgb(5, 2, 5),
		Color::Crimson => termion::color::AnsiValue::rgb(4, 0, 0),
		Color::LavenderBlush => termion::color::AnsiValue::rgb(5, 4, 5),
		Color::PaleVioletRed => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::HotPink => termion::color::AnsiValue::rgb(4, 0, 4),
		Color::DeepPink => termion::color::AnsiValue::rgb(4, 0, 2),
		Color::MediumVioletRed => termion::color::AnsiValue::rgb(2, 0, 1),
		Color::Orchid => termion::color::AnsiValue::rgb(3, 0, 5),
		Color::Thistle => termion::color::AnsiValue::grayscale(14),
		Color::Plum => termion::color::AnsiValue::rgb(3, 2, 5),
		Color::Violet => termion::color::AnsiValue::rgb(5, 1, 5),
		Color::Magenta => termion::color::AnsiValue::rgb(5, 0, 4),
		Color::Fuchsia => termion::color::AnsiValue::rgb(5, 0, 4),
		Color::DarkMagenta => termion::color::AnsiValue::rgb(1, 0, 1),
		Color::Purple => termion::color::AnsiValue::rgb(1, 0, 1),
		Color::MediumOrchid => termion::color::AnsiValue::rgb(2, 0, 3),
		Color::DarkViolet => termion::color::AnsiValue::rgb(2, 0, 1),
		Color::DarkOrchid => termion::color::AnsiValue::rgb(2, 0, 1),
		Color::Indigo => termion::color::AnsiValue::rgb(0, 0, 1),
		Color::BlueViolet => termion::color::AnsiValue::rgb(2, 0, 1),
		Color::MediumPurple => termion::color::AnsiValue::rgb(1, 1, 5),
		Color::MediumSlateBlue => termion::color::AnsiValue::rgb(0, 1, 5),
		Color::SlateBlue => termion::color::AnsiValue::rgb(0, 1, 5),
		Color::DarkSlateBlue => termion::color::AnsiValue::rgb(0, 0, 1),
		Color::Lavender => termion::color::AnsiValue::grayscale(17),
		Color::GhostWhite => termion::color::AnsiValue::grayscale(22),
		Color::Blue => termion::color::AnsiValue::rgb(0, 0, 4),
		Color::MediumBlue => termion::color::AnsiValue::rgb(0, 0, 1),
		Color::MidnightBlue => termion::color::AnsiValue::rgb(0, 0, 1),
		Color::DarkBlue => termion::color::AnsiValue::rgb(0, 0, 1),
		Color::Navy => termion::color::AnsiValue::rgb(0, 0, 1),
		Color::RoyalBlue => termion::color::AnsiValue::rgb(0, 0, 5),
		Color::CornflowerBlue => termion::color::AnsiValue::rgb(0, 1, 5),
		Color::LightSteelBlue => termion::color::AnsiValue::rgb(2, 4, 5),
		Color::LightSlateGray => termion::color::AnsiValue::grayscale(5),
		Color::SlateGray => termion::color::AnsiValue::grayscale(5),
		Color::DodgerBlue => termion::color::AnsiValue::rgb(0, 1, 5),
		Color::AliceBlue => termion::color::AnsiValue::rgb(2, 5, 5),
		Color::SteelBlue => termion::color::AnsiValue::rgb(0, 2, 3),
		Color::LightSkyBlue => termion::color::AnsiValue::rgb(0, 4, 5),
		Color::SkyBlue => termion::color::AnsiValue::rgb(0, 4, 5),
		Color::DeepSkyBlue => termion::color::AnsiValue::rgb(0, 1, 5),
		Color::LightBlue => termion::color::AnsiValue::rgb(1, 5, 5),
		Color::PowderBlue => termion::color::AnsiValue::rgb(1, 5, 5),
		Color::CadetBlue => termion::color::AnsiValue::rgb(0, 1, 1),
		Color::Azure => termion::color::AnsiValue::rgb(4, 5, 5),
		Color::LightCyan => termion::color::AnsiValue::rgb(3, 5, 5),
		Color::PaleTurquoise => termion::color::AnsiValue::rgb(1, 5, 5),
		Color::Cyan => termion::color::AnsiValue::rgb(1, 3, 5),
		Color::Aqua => termion::color::AnsiValue::rgb(1, 3, 5),
		Color::DarkTurquoise => termion::color::AnsiValue::rgb(0, 2, 2),
		Color::DarkSlateGray => termion::color::AnsiValue::grayscale(1),
		Color::DarkCyan => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::Teal => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::MediumTurquoise => termion::color::AnsiValue::rgb(0, 3, 2),
		Color::LightSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::Turquoise => termion::color::AnsiValue::rgb(0, 3, 2),
		Color::Aquamarine => termion::color::AnsiValue::rgb(0, 5, 3),
		Color::MediumAquamarine => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::MediumSpringGreen => termion::color::AnsiValue::rgb(0, 5, 0),
		Color::MintCream => termion::color::AnsiValue::grayscale(23),
		Color::SpringGreen => termion::color::AnsiValue::rgb(0, 5, 0),
		Color::MediumSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::SeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::Honeydew => termion::color::AnsiValue::rgb(2, 5, 2),
		Color::LightGreen => termion::color::AnsiValue::rgb(0, 4, 0),
		Color::PaleGreen => termion::color::AnsiValue::rgb(0, 4, 0),
		Color::DarkSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::LimeGreen => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::Lime => termion::color::AnsiValue::rgb(0, 4, 0),
		Color::ForestGreen => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::Green => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::DarkGreen => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::Chartreuse => termion::color::AnsiValue::rgb(0, 4, 0),
		Color::LawnGreen => termion::color::AnsiValue::rgb(0, 4, 0),
		Color::GreenYellow => termion::color::AnsiValue::rgb(0, 4, 0),
		Color::DarkOliveGreen => termion::color::AnsiValue::rgb(0, 1, 0),
		Color::YellowGreen => termion::color::AnsiValue::rgb(1, 2, 0),
		Color::OliveDrab => termion::color::AnsiValue::rgb(0, 2, 0),
		Color::Beige => termion::color::AnsiValue::rgb(3, 3, 5),
		Color::LightGoldenrodYellow => termion::color::AnsiValue::rgb(3, 3, 5),
		Color::Ivory => termion::color::AnsiValue::rgb(5, 5, 4),
		Color::LightYellow => termion::color::AnsiValue::rgb(5, 5, 4),
		Color::Yellow => termion::color::AnsiValue::rgb(5, 5, 0),
		Color::Olive => termion::color::AnsiValue::rgb(2, 1, 0),
		Color::DarkKhaki => termion::color::AnsiValue::rgb(2, 1, 1),
		Color::LemonChiffon => termion::color::AnsiValue::rgb(4, 4, 2),
		Color::PaleGoldenrod => termion::color::AnsiValue::rgb(3, 3, 1),
		Color::Khaki => termion::color::AnsiValue::rgb(3, 3, 1),
		Color::Gold => termion::color::AnsiValue::rgb(4, 2, 0),
		Color::Cornsilk => termion::color::AnsiValue::rgb(4, 4, 2),
		Color::Goldenrod => termion::color::AnsiValue::rgb(2, 1, 0),
		Color::DarkGoldenrod => termion::color::AnsiValue::rgb(1, 1, 0),
		Color::FloralWhite => termion::color::AnsiValue::grayscale(23),
		Color::OldLace => termion::color::AnsiValue::grayscale(22),
		Color::Wheat => termion::color::AnsiValue::rgb(5, 3, 2),
		Color::Moccasin => termion::color::AnsiValue::rgb(5, 3, 2),
		Color::Orange => termion::color::AnsiValue::rgb(5, 1, 0),
		Color::PapayaWhip => termion::color::AnsiValue::rgb(5, 3, 3),
		Color::BlanchedAlmond => termion::color::AnsiValue::rgb(5, 3, 3),
		Color::NavajoWhite => termion::color::AnsiValue::rgb(5, 3, 2),
		Color::AntiqueWhite => termion::color::AnsiValue::rgb(5, 5, 5),
		Color::Tan => termion::color::AnsiValue::rgb(2, 1, 1),
		Color::BurlyWood => termion::color::AnsiValue::rgb(2, 1, 1),
		Color::Bisque => termion::color::AnsiValue::rgb(3, 1, 0),
		Color::DarkOrange => termion::color::AnsiValue::rgb(5, 1, 0),
		Color::Linen => termion::color::AnsiValue::grayscale(21),
		Color::Peru => termion::color::AnsiValue::rgb(1, 1, 0),
		Color::PeachPuff => termion::color::AnsiValue::rgb(3, 1, 0),
		Color::SandyBrown => termion::color::AnsiValue::rgb(3, 1, 0),
		Color::Chocolate => termion::color::AnsiValue::rgb(2, 0, 0),
		Color::SaddleBrown => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::Seashell => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::Sienna => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::LightSalmon => termion::color::AnsiValue::rgb(4, 1, 1),
		Color::Coral => termion::color::AnsiValue::rgb(4, 1, 1),
		Color::OrangeRed => termion::color::AnsiValue::rgb(4, 0, 0),
		Color::DarkSalmon => termion::color::AnsiValue::rgb(3, 1, 0),
		Color::Tomato => termion::color::AnsiValue::rgb(3, 0, 0),
		Color::MistyRose => termion::color::AnsiValue::rgb(5, 1, 1),
		Color::Salmon => termion::color::AnsiValue::rgb(4, 1, 0),
		Color::Snow => termion::color::AnsiValue::grayscale(23),
		Color::LightCoral => termion::color::AnsiValue::rgb(4, 1, 0),
		Color::RosyBrown => termion::color::AnsiValue::rgb(1, 1, 1),
		Color::IndianRed => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::Red => termion::color::AnsiValue::rgb(4, 0, 0),
		Color::Brown => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::FireBrick => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::DarkRed => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::Maroon => termion::color::AnsiValue::rgb(1, 0, 0),
		Color::White => termion::color::AnsiValue::grayscale(23),
		Color::WhiteSmoke => termion::color::AnsiValue::grayscale(19),
		Color::Gainsboro => termion::color::AnsiValue::grayscale(17),
		Color::LightGrey => termion::color::AnsiValue::grayscale(16),
		Color::Silver => termion::color::AnsiValue::grayscale(14),
		Color::DarkGray => termion::color::AnsiValue::grayscale(8),
		Color::Gray => termion::color::AnsiValue::grayscale(4),
		Color::DimGray => termion::color::AnsiValue::grayscale(3),
		Color::Black => termion::color::AnsiValue::grayscale(0),
	}
}
