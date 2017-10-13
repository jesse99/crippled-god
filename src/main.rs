extern crate termion;

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

struct Square {
    symbol: char,
    back_color: termion::color::AnsiValue, // see https://github.com/jbnicolai/ansi-256-colors
    fore_color: termion::color::AnsiValue, // note that the Mac terminal doesn't support true color, aka termion::color::Rgb
}

const TERRAIN: &'static str = r#"
##############################################################
#                                                            #
#                                                            #
#                                                            #
#                                              w             #
#                                             www            #
#                                            wwwww           #
#                                              ww            #
#                                                            #
#============================                                #
#                           =                                #
#                           =                                #
#                           =                                #
#                           =                                #
#                           =                                #
#                           =                                #
#                           =                                #
#                           =                                #
##############################################################"#;

fn fatal_err(message: &str) -> ! {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}\n", message);
    write!(
        stdout,
        "{}{}\n",
        termion::cursor::Restore,
        termion::cursor::Show
    );
    panic!();
}

macro_rules! fatal_error_if
{
	($predicate:expr) => (if $predicate {fatal_err("")});
	($predicate:expr, $msg:expr) => (if $predicate {fatal_err($msg)});
	($predicate:expr, $fmt:expr, $($arg:tt)*) => (if $predicate {fatal_err(&format!($fmt, $($arg)*))});
}

fn build_map(text: &str) -> Vec<Vec<Square>> {
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
                    back_color: termion::color::AnsiValue::rgb(1, 0, 0),
                    fore_color: termion::color::AnsiValue::grayscale(0),
                })
            }
            '=' => {
                row.push(Square {
                    symbol: ' ',
                    back_color: termion::color::AnsiValue::rgb(3, 1, 0),
                    fore_color: termion::color::AnsiValue::grayscale(0),
                })
            }
            'w' => {
                row.push(Square {
                    symbol: ' ',
                    back_color: termion::color::AnsiValue::rgb(0, 0, 4),
                    fore_color: termion::color::AnsiValue::grayscale(0),
                })
            }
            _ => {
                row.push(Square {
                    symbol: c,
                    back_color: termion::color::AnsiValue::grayscale(0),
                    fore_color: termion::color::AnsiValue::grayscale(0),
                })
            }
        }
    }
    map.push(row);

    map
}

fn render_map(stdout: &mut RawTerminal, map: &Vec<Vec<Square>>, player_x: usize, player_y: usize) {
    let mut y = 1; // terminal coordinates are 1-based
    for row in map.iter() {
        for (x, s) in row.iter().enumerate() {
            let c = if x == player_x && y == player_y + 1 {
                '@'
            } else {
                s.symbol
            };
            let f = if x == player_x && y == player_y + 1 {
                termion::color::AnsiValue::grayscale(23)
            } else {
                s.fore_color
            };
            write!(
                stdout,
                "\n{}{}{}{}",
                termion::cursor::Goto((x + 1) as u16, y as u16),
                termion::color::Fg(f),
                termion::color::Bg(s.back_color),
                c
            );
        }
        y += 1;
    }
    stdout.flush().unwrap();
}

fn main() {
    // let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
    // println!("width = {}, height = {}", width, height);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    write!(stdout, "\n{}{}", termion::cursor::Hide, termion::clear::All);

    let map = build_map(TERRAIN);
    let mut player_x = 5;
    let mut player_y = 5;

    render_map(&mut stdout, &map, player_x, player_y);
    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('q') => break,
            termion::event::Key::Left => player_x -= 1,
            termion::event::Key::Right => player_x += 1,
            termion::event::Key::Up => player_y -= 1,
            termion::event::Key::Down => player_y += 1,
            _ => {
                write!(stdout, "\x07");
            }
        };
        render_map(&mut stdout, &map, player_x, player_y);
    }

    write!(
        stdout,
        "\n{}{}",
        termion::cursor::Restore,
        termion::cursor::Show
    );
    stdout.flush().unwrap();
}
