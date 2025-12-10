use std::{cmp, collections::VecDeque, convert::identity};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub from: Point,
    pub to: Point,
}

impl Rectangle {
    pub fn area(&self) -> u64 {
        let dx = self.from.x.abs_diff(self.to.x.into()) + 1;
        let dy = self.from.y.abs_diff(self.to.y.into()) + 1;

        dx * dy
    }
}

pub struct Grid {
    xs: Vec<u64>,
    ys: Vec<u64>,
    grid: Vec<Vec<bool>>,
}

impl Grid {
    pub fn build<'a>(points: &'a [Point]) -> Self {
        let mut xs: Vec<u64> = points.iter().map(|p| p.x).collect();
        xs.push(0); // Add synthetic (0, 0)
        xs.sort_unstable();
        xs.dedup();

        xs = Grid::add_spacers(&xs);

        xs.push(xs.last().expect("can't be empty") + 1);

        let mut ys: Vec<u64> = points.iter().map(|p| p.y).collect();
        ys.push(0); // Add synthetic (0, 0)
        ys.sort_unstable();
        ys.dedup();

        ys = Grid::add_spacers(&ys);

        ys.push(ys.last().expect("can't be empty") + 1);

        let mut grid = vec![vec![true; ys.len()]; xs.len()];
        let mut border = vec![vec![false; ys.len()]; xs.len()];

        eprintln!("initialising border {} {}", xs.len(), ys.len());
        Grid::init_border(&mut border, &xs, &ys, points);
        eprintln!("initialised!");

        eprintln!("border:");
        Grid::print_grid(&border, &ys);

        Grid::fill_grid(&mut grid, &border);

        eprintln!("grid:");
        Grid::print_grid(&grid, &ys);

        Self { xs, ys, grid }
    }

    fn add_spacers(xs: &[u64]) -> Vec<u64> {
        xs.iter().enumerate().flat_map(|( i, &x )| {
            if i == xs.len() - 1 {
                return vec![x]
            }
            let next = xs[i + 1];
            if next > x + 1 {
                return vec![x, x + 1];
            }
            return vec![x];
        }).collect()
    }

    fn print_grid(grid: &Vec<Vec<bool>>, ys: &[u64]) {
        for y_idx in (0..ys.len()).rev() {
            for x_idx in 0..grid.len() {
                eprint!("{}", if grid[x_idx][y_idx] { '#' } else { '.' });
            }
            eprintln!();
        }
    }

    fn init_border(border: &mut Vec<Vec<bool>>, xs: &[u64], ys: &[u64], points: &[Point]) -> () {
        for i in 0..points.len() {
            let from = i;
            let to = if i == points.len() - 1 { 0 } else { i + 1 };

            let p1 = points[from];
            let p2 = points[to];

            eprintln!("{} -> {}, {:?} -> {:?}", from, to, p1, p2);

            if p1.x == p2.x {
                let x_idx = xs.binary_search(&p1.x).expect("x_idx has to be on the grid");

                let from_y_idx = ys.binary_search(&cmp::min(p1.y, p2.y)).expect("y_idx has to be on the grid");
                let to_y_idx = ys.binary_search(&cmp::max(p1.y, p2.y)).expect("y_idx has to be on the grid");
                for y_idx in from_y_idx..=to_y_idx {
                    border[x_idx][y_idx] = true;
                }
            } else if p1.y == p2.y {
                let y_idx = ys.binary_search(&p1.y).expect("y_idx has to be on the grid");

                let from_x_idx = xs.binary_search(&cmp::min(p1.x, p2.x)).expect("x_idx has to be on the grid");
                let to_x_idx = xs.binary_search(&cmp::max(p1.x, p2.x)).expect("x_idx has to be on the grid");
                for x_idx in from_x_idx..=to_x_idx {
                    border[x_idx][y_idx] = true;
                }
            } else {
                panic!("consecutive points are not on the same X or Y axis");
            }
        }
    }

    fn fill_grid(grid: &mut Vec<Vec<bool>>, border: &Vec<Vec<bool>>) -> () {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0usize, 0usize)); // Start from the synthetic start

        while let Some((x, y)) = queue.pop_front() {
            if !grid[x][y] {
                continue;
            }
            grid[x][y] = false;
            let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in dirs {
                let to_x = x as isize + dx;
                let to_y = y as isize + dy;
                if !(to_x >= 0 && to_y >= 0) {
                    continue;
                }
                let to_x = to_x as usize;
                let to_y = to_y as usize;
                if !(to_x < grid.len() && to_y < grid[0].len()) {
                    continue;
                }

                if grid[to_x][to_y] && !border[to_x][to_y] {
                    queue.push_back((to_x, to_y));
                }
            }
        }
    }

    pub fn is_inner(&self, rect: &Rectangle) -> bool {
        let from_x_idx = self
            .xs
            .binary_search(&rect.from.x)
            .expect("has to be on the grid");
        let to_x_idx = self
            .xs
            .binary_search(&rect.to.x)
            .expect("has to be on the grid");
        let from_y_idx = self
            .ys
            .binary_search(&rect.from.y)
            .expect("has to be on the grid");
        let to_y_idx = self
            .ys
            .binary_search(&rect.to.y)
            .expect("has to be on the grid");

        let min_x_idx = cmp::min(from_x_idx, to_x_idx);
        let max_x_idx = cmp::max(from_x_idx, to_x_idx);
        let min_y_idx = cmp::min(from_y_idx, to_y_idx);
        let max_y_idx = cmp::max(from_y_idx, to_y_idx);

        (min_x_idx..=max_x_idx)
            .flat_map(|x_idx| (min_y_idx..=max_y_idx).map(move |y_idx| self.grid[x_idx][y_idx]))
            .all(identity)
    }
}

pub fn find_biggest_rectangle(points: &[Point]) -> Rectangle {
    let pairs = (0..(points.len() - 1)).flat_map(|i| ((i + 1)..points.len()).map(move |j| (i, j)));

    let max_by_area = pairs
        .max_by(|x, y| {
            let x_rect = Rectangle {
                from: points[x.0],
                to: points[x.1],
            };
            let y_rect = Rectangle {
                from: points[y.0],
                to: points[y.1],
            };
            x_rect.area().cmp(&y_rect.area())
        })
        .expect("points can't be empty");

    Rectangle {
        from: points[max_by_area.0],
        to: points[max_by_area.1],
    }
}

pub fn find_biggest_inner_rectangle(points: &[Point]) -> Option<Rectangle> {
    let pairs = (0..(points.len() - 1)).flat_map(|i| ((i + 1)..points.len()).map(move |j| (i, j)));

    let to_rect = |(from_idx, to_idx): &(usize, usize)| Rectangle {
        from: points[*from_idx],
        to: points[*to_idx],
    };

    eprintln!("building the grid...");
    let grid = Grid::build(points);
    eprintln!("done!");

    let inner_pairs = pairs.filter(|pair| {
        eprintln!("analyzing pair... {:?}", pair);
        grid.is_inner(&to_rect(pair))
    });

    let max_by_area = inner_pairs.map(|x| {
        let r = to_rect(&x);
        eprintln!("inner pair {:?} {:?} {:?}", x, r, r.area());
        x
    }).max_by(|x, y| {
        let x_rect = to_rect(x);
        let y_rect = to_rect(y);
        x_rect.area().cmp(&y_rect.area())
    })?;

    Some(Rectangle {
        from: points[max_by_area.0],
        to: points[max_by_area.1],
    })
}

//      x=5    x=10
// y=10  +------+
//       |      |
//       |      |
// y=5   +------+
#[test]
fn inner_rectangle_simple() {
    let points = vec![
        Point { x: 5, y: 10 },
        Point { x: 10, y: 10 },
        Point { x: 10, y: 5 },
        Point { x: 5, y: 5 },
    ];

    let result = find_biggest_inner_rectangle(&points).expect("should find a rectangle");

    let mut xs = [result.from.x, result.to.x];
    let mut ys = [result.from.y, result.to.y];
    xs.sort();
    ys.sort();

    assert_eq!(xs, [5, 10]);
    assert_eq!(ys, [5, 10]);
}

//       x=5  x=10 x=11 x=12
// y=12       +----+----+
//            |         |
// y=10  +----+    +----+
//       |         |
//       |         |
// y=5   +---------+
#[test]
fn inner_rectangle_l_shape() {
    let points = vec![
        Point { x: 5, y: 5 },
        Point { x: 5, y: 10 },
        Point { x: 10, y: 10 },
        Point { x: 10, y: 12 },
        Point { x: 12, y: 12 },
        Point { x: 12, y: 10 },
        Point { x: 11, y: 10 },
        Point { x: 11, y: 5 },
    ];

    let result = find_biggest_inner_rectangle(&points).expect("should find a rectangle");

    let mut xs = [result.from.x, result.to.x];
    let mut ys = [result.from.y, result.to.y];
    xs.sort();
    ys.sort();

    assert_eq!(xs, [5, 11]);
    assert_eq!(ys, [5, 10]);
}

//       x=5  x=10 x=12   x=17
// y=13  +----------------+
//       |                |
// y=12  +----+     +-----+
//            |     |
// y=10  +----+     +-----+
//       |                |
//       |                |
// y=5   +----------------+
#[test]
fn inner_rectangle_h_shape() {
    let points = vec![
        Point { x: 5, y: 5 },
        Point { x: 5, y: 10 },
        Point { x: 10, y: 10 },
        Point { x: 10, y: 12 },
        Point { x: 5, y: 12 },
        Point { x: 5, y: 13 },
        Point { x: 17, y: 13 },
        Point { x: 17, y: 12 },
        Point { x: 12, y: 12 },
        Point { x: 12, y: 10 },
        Point { x: 17, y: 10 },
        Point { x: 17, y: 5 },
    ];

    let result = find_biggest_inner_rectangle(&points).expect("should find a rectangle");

    let mut xs = [result.from.x, result.to.x];
    let mut ys = [result.from.y, result.to.y];
    xs.sort();
    ys.sort();

    assert_eq!(xs, [5, 17]);
    assert_eq!(ys, [5, 10]);
}

//       x=5  x=10 x=12   x=17
// y=13  +----------------+
//       |                |
// y=12  +----+     +-----+
//            |     |
// y=10  +----+     +-----+
//       |                |
//       |                |
// y=5   +----------------+
#[test]
fn grid_h_shape_queries() {
    let points = vec![
        Point { x: 5, y: 5 },
        Point { x: 5, y: 10 },
        Point { x: 10, y: 10 },
        Point { x: 10, y: 12 },
        Point { x: 5, y: 12 },
        Point { x: 5, y: 13 },
        Point { x: 17, y: 13 },
        Point { x: 17, y: 12 },
        Point { x: 12, y: 12 },
        Point { x: 12, y: 10 },
        Point { x: 17, y: 10 },
        Point { x: 17, y: 5 },
    ];

    let grid = Grid::build(&points);

    // full shape - includes the gap
    assert_eq!(
        grid.is_inner(&Rectangle {
            from: Point { x: 5, y: 5 },
            to: Point { x: 17, y: 13 },
        }),
        false
    );

    // vertical line - there's a gap
    assert_eq!(
        grid.is_inner(&Rectangle {
            from: Point { x: 5, y: 5 },
            to: Point { x: 5, y: 13 },
        }),
        false
    );

    // along the top edge
    assert_eq!(
        grid.is_inner(&Rectangle {
            from: Point { x: 12, y: 10 },
            to: Point { x: 17, y: 13 },
        }),
        false
    );

    // contained inside
    assert_eq!(
        grid.is_inner(&Rectangle {
            from: Point { x: 5, y: 5 },
            to: Point { x: 12, y: 10 },
        }),
        true
    );
}
