use engine::map;
#[macro_use]
use log;
use rand;
use rand::Rng;

pub fn generate_open(rng: &mut rand::StdRng) -> map::Map {
    // pick a random width and height
    let width: usize = rng.gen_range(20, 100);
    let height: usize = rng.gen_range(20, 35);

    let open = map::Square {
        symbol: '.',
        back_color: map::Color::LightGrey,
        fore_color: map::Color::LightGrey,
    };
    let mut map = map::Map::with_size(width, height, &open);

    // add walls on the edges
    let wall = map::Square {
        symbol: '#',
        back_color: map::Color::SaddleBrown,
        fore_color: map::Color::SaddleBrown,
    };
    for y in 0..height {
        map.set_square(0, y, &wall);
        for x in 0..width {
            map.set_square(x, 0, &wall);
            map.set_square(x, height - 1, &wall);
        }
        map.set_square(width - 1, y, &wall);
    }

    // plop down some water
    while rng.gen_weighted_bool(2) {
        add_lake(&mut map, rng);
    }

    debug!("generated open map:");
    debug!("\n{:?}", map);

    // maybe also a road or two
    map
}

fn add_lake(map: &mut map::Map, rng: &mut rand::StdRng) {
    let center_x = rng.gen_range(1, map.width) as f64;
    let center_y = rng.gen_range(1, map.height) as f64;
    let diameter: usize = rng.gen_range(2, 10);

    let water = map::Square {
        symbol: 'w',
        back_color: map::Color::DodgerBlue,
        fore_color: map::Color::DodgerBlue,
    };
    for v in 0..diameter {
        let y = center_y - diameter as f64 / 2.0 + v as f64;
        if y > 1.0 && y + 1.0 < map.height as f64 {
            let dy = y - center_y;
            for h in 0..diameter {
                let x = center_x - diameter as f64 / 2.0 + h as f64;
                if x > 1.0 && x + 1.0 < map.width as f64 {
                    let dx = x - center_x;
                    let dist = (dx * dx + dy * dy).sqrt();
                    let dist = dist / (diameter as f64);
                    if rng.next_f64() >= 0.5 * dist {
                        map.set_square(x as usize, y as usize, &water);
                    }
                }
            }
        }
    }
}
