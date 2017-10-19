use engine::map;
use game;
// #[macro_use]
// use log;
use rand;
use rand::Rng;

pub fn generate_open(rng: &mut rand::StdRng) -> map::Map {
    // pick a random width and height
    let width: i32 = rng.gen_range(20, 100);
    let height: i32 = rng.gen_range(20, 35);

    let open = map::Square {
        terrain: game::Terrain::Ground,
        feature: None,
    };
    let mut map = map::Map::with_size(width, height, &open);

    // add walls on the edges
    let wall = map::Square {
        terrain: game::Terrain::Wall,
        feature: None,
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

    // plop down some stairs
    add_stair(&mut map, rng, game::Feature::DownStair);
    add_stair(&mut map, rng, game::Feature::DownStair);

    debug!("generated open map:");
    debug!("\n{:?}", map);

    // maybe also a road or two
    map
}

fn add_lake(map: &mut map::Map, rng: &mut rand::StdRng) {
    let center_x = rng.gen_range(1, map.width) as f64;
    let center_y = rng.gen_range(1, map.height) as f64;
    let diameter: i32 = rng.gen_range(2, 10);

    let water = map::Square {
        terrain: game::Terrain::Water,
        feature: None,
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
                        map.set_square(x as i32, y as i32, &water);
                    }
                }
            }
        }
    }
}

fn add_stair(map: &mut map::Map, rng: &mut rand::StdRng, stair: game::Feature) {
    let mut xs: Vec<i32> = (0..map.width).collect();
    let mut ys: Vec<i32> = (0..map.height).collect();

    rng.shuffle(&mut xs);
    rng.shuffle(&mut ys);

    while !xs.is_empty() && !ys.is_empty() {
        let x = xs.pop().unwrap();
        let y = ys.pop().unwrap();

        let mut square = *map.get_square(x, y);
        if let game::Terrain::Ground = square.terrain {
            if let None = square.feature {
                square.feature = Some(stair);
                map.set_square(x, y, &square);
                return;
            }
        }
    }

    error!("Couldn't add a stair");
}
