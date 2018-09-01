// This is used to figure out which cells are visible by a character.
// There are lots of ways to do this (see http://www.roguebasin.com/index.php?title=Category:FOV)
// but here we are using the Precise Permissive Field of View algorithm based on the Python
// code at http://www.roguebasin.com/index.php?title=Permissive_Field_of_View_in_Python by
// Aaron MacDonald.
use super::location::Location;
use super::size::Size;
use std::collections::HashSet;

/// Calls visit_tile for each cell that is visible from start.
///
/// # Arguments
///
/// * `start` - Where to start checking for visiblr cells from. Typically the position of a character.
/// * `size` - How many cells to check. Typically the size of the level.
/// * `radius` - Maximum distance that LOS can extend to.
/// * `visit_tile` - Called for each visible cell.
/// * `blocks_LOS` - Returns true if the cell blocks LOS.
pub fn visit_visible_cells<V, B>(
	start: Location,
	size: Size,
	radius: i32,
	mut visit_tile: V,
	blocks_LOS: B,
) where
	V: FnMut(Location),
	B: Fn(Location) -> bool, // TODO: this will probably need to take a some sort of trait, race? character size?
{
	// If the starting point cannot be seen then the character is presumbably blinded so visit nothing.
	if blocks_LOS(start) {
		return;
	}

	let mut visited = HashSet::new(); // TODO: don't use a cryptograhic hash
	visit_tile(start);
	visited.insert(start);

	// Get the dimensions of the actual field of view, making sure not to go off the map or beyond the radius.
	let min_extent_x = if start.x < radius { start.x } else { radius };
	let max_extent_x = if size.width - start.x - 1 < radius {
		size.width - start.x - 1
	} else {
		radius
	};

	let min_extent_y = if start.y < radius { start.y } else { radius };
	let max_extent_y = if size.height - start.y - 1 < radius {
		size.height - start.y - 1
	} else {
		radius
	};

	// Northeast quadrant
	check_quadrant(
		&mut visited,
		start,
		Location::new(1, 1),
		max_extent_x,
		max_extent_y,
		&mut visit_tile,
		&blocks_LOS,
	);

	// Southeast quadrant
	check_quadrant(
		&mut visited,
		start,
		Location::new(1, -1),
		max_extent_x,
		min_extent_y,
		&mut visit_tile,
		&blocks_LOS,
	);

	// Southwest quadrant
	check_quadrant(
		&mut visited,
		start,
		Location::new(-1, -1),
		min_extent_x,
		min_extent_y,
		&mut visit_tile,
		&blocks_LOS,
	);

	// Northwest quadrant
	check_quadrant(
		&mut visited,
		start,
		Location::new(-1, 1),
		min_extent_x,
		max_extent_y,
		&mut visit_tile,
		&blocks_LOS,
	);
}

// ---- Private Items -------------------------------------------------------------------
#[derive(Clone, Copy)]
struct Line {
	i: Location,
	f: Location,
}

impl Line {
	fn new(ix: i32, iy: i32, fx: i32, fy: i32) -> Line {
		let i = Location::new(ix, iy);
		let f = Location::new(fx, fy);
		Line { i, f }
	}

	fn dx(&self) -> i32 {
		self.f.x - self.i.x
	}

	fn dy(&self) -> i32 {
		self.f.y - self.i.y
	}

	fn below(&self, loc: Location) -> bool {
		self.relative_slope(loc) > 0
	}

	fn below_or_collinear(&self, loc: Location) -> bool {
		self.relative_slope(loc) >= 0
	}

	fn above(&self, loc: Location) -> bool {
		self.relative_slope(loc) < 0
	}

	fn above_or_collinear(&self, loc: Location) -> bool {
		self.relative_slope(loc) <= 0
	}

	fn collinear_point(&self, x: i32, y: i32) -> bool {
		self.relative_slope(Location::new(x, y)) == 0
	}

	fn collinear_line(&self, line: &Line) -> bool {
		self.collinear_point(line.i.x, line.i.y) && self.collinear_point(line.f.x, line.f.y)
	}

	fn relative_slope(&self, loc: Location) -> i32 {
		(self.dy() * (self.f.x - loc.x)) - (self.dx() * (self.f.y - loc.y))
	}
}

#[derive(Clone)]
struct View {
	shallow_line: Line,
	steep_line: Line,
	shallow_bump: Vec<Location>,
	steep_bump: Vec<Location>,
}

impl View {
	fn new(shallow_line: Line, steep_line: Line) -> View {
		View {
			shallow_line,
			steep_line,
			shallow_bump: Vec::new(),
			steep_bump: Vec::new(),
		}
	}
}

fn check_quadrant<V, B>(
	visited: &mut HashSet<Location>,
	start: Location,
	delta: Location,
	extent_x: i32,
	extent_y: i32,
	visit_tile: &mut V,
	blocks_LOS: &B,
) where
	V: FnMut(Location),
	B: Fn(Location) -> bool,
{
	let mut active_views = Vec::new();

	let shallow_line = Line::new(0, 1, extent_x, 0);
	let steep_line = Line::new(1, 0, 0, extent_y);

	active_views.push(View::new(shallow_line, steep_line));
	let view_index = 0;

	// Visit the tiles diagonally and going outwards
	//
	// .
	// .
	// .           .
	// 9        .
	// 5  8  .
	// 2  4  7
	// @  1  3  6  .  .  .
	let max_i = extent_x + extent_y;
	let mut i = 1;
	while i != max_i + 1 && !active_views.is_empty() {
		let start_j = if 0 > i - extent_x { 0 } else { i - extent_x };
		let max_j = if i < extent_y { i } else { extent_y };

		let mut j = start_j;

		while j != max_j + 1 && view_index < active_views.len() {
			let x = i - j;
			let y = j;
			visit_coord(
				visited,
				start,
				x,
				y,
				delta,
				view_index,
				&mut active_views,
				visit_tile,
				blocks_LOS,
			);

			j += 1;
		}

		i += 1;
	}
}

fn visit_coord<V, B>(
	visited: &mut HashSet<Location>,
	start: Location,
	x: i32,
	y: i32,
	delta: Location,
	view_index: usize,
	active_views: &mut Vec<View>,
	visit_tile: &mut V,
	blocks_LOS: &B,
) where
	V: FnMut(Location),
	B: Fn(Location) -> bool,
{
	let mut view_index = view_index;

	// The top left and bottom right corners of the current coordinate.
	let topLeft = Location::new(x, y + 1);
	let bottom_right = Location::new(x + 1, y);

	while view_index < active_views.len()
		&& active_views[view_index]
			.steep_line
			.below_or_collinear(bottom_right)
	{
		// The current coordinate is above the current view and is
		// ignored.  The steeper fields may need it though.
		view_index += 1
	}

	if view_index == active_views.len()
		|| active_views[view_index]
			.shallow_line
			.above_or_collinear(topLeft)
	{
		// Either the current coordinate is above all of the fields
		// or it is below all of the fields.
		return;
	}

	// It is now known that the current coordinate is between the steep
	// and shallow lines of the current view.

	// The real quadrant coordinates
	let real_x = x * delta.x;
	let real_y = y * delta.y;

	let loc = Location::new(start.x + real_x, start.y + real_y);
	if !visited.contains(&loc) {
		visited.insert(loc);
		visit_tile(loc);
		// } else {
		// 	println!("{?:}", loc);
	}

	if !blocks_LOS(loc) {
		// The current coordinate does not block sight and therefore
		// has no effect on the view.
		return;
	}

	if active_views[view_index].shallow_line.above(bottom_right)
		&& active_views[view_index].steep_line.below(topLeft)
	{
		// The current coordinate is intersected by both lines in the
		// current view.  The view is completely blocked.
		active_views.remove(view_index);
	} else if active_views[view_index].shallow_line.above(bottom_right) {
		// The current coordinate is intersected by the shallow line of
		// the current view.  The shallow line needs to be raised.
		add_shallow_bump(topLeft, active_views, view_index);
		check_view(active_views, view_index);
	} else if active_views[view_index].steep_line.below(topLeft) {
		// The current coordinate is intersected by the steep line of
		// the current view.  The steep line needs to be lowered.
		add_steep_bump(bottom_right, active_views, view_index);
		check_view(active_views, view_index);
	} else {
		// The current coordinate is completely between the two lines
		// of the current view.  Split the current view into two views
		// above and below the current coordinate.
		let shallow_view_index = view_index;
		view_index += 1;
		let mut steep_view_index = view_index;

		let copy = active_views[shallow_view_index].clone();
		active_views.insert(shallow_view_index, copy);

		add_steep_bump(bottom_right, active_views, shallow_view_index);
		if !check_view(active_views, shallow_view_index) {
			//view_index -= 1;			// TODO: why did Python have this?
			steep_view_index -= 1;
		}

		add_shallow_bump(topLeft, active_views, steep_view_index);
		check_view(active_views, steep_view_index);
	}
}

fn add_shallow_bump(loc: Location, active_views: &mut Vec<View>, view_index: usize) {
	let view = &mut active_views[view_index];

	view.shallow_line.f = loc;
	view.shallow_bump.insert(0, loc);

	for bump in view.steep_bump.iter() {
		if view.shallow_line.above(*bump) {
			view.shallow_line.i = *bump;
		}
	}
}

fn add_steep_bump(loc: Location, active_views: &mut Vec<View>, view_index: usize) {
	let view = &mut active_views[view_index];

	view.steep_line.f = loc;
	view.steep_bump.insert(0, loc);

	for bump in view.shallow_bump.iter() {
		if view.steep_line.below(*bump) {
			view.steep_line.i = *bump;
		}
	}
}

// Removes the view in active_views at index view_index if
//    - The two lines are coolinear
//    - The lines pass through either extremity
fn check_view(active_views: &mut Vec<View>, view_index: usize) -> bool {
	let shallow_line = active_views[view_index].shallow_line;
	let steep_line = active_views[view_index].steep_line;

	if shallow_line.collinear_line(&steep_line)
		&& (shallow_line.collinear_point(0, 1) || shallow_line.collinear_point(1, 0))
	{
		active_views.remove(view_index);
		false
	} else {
		true
	}
}
