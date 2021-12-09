use im_rc::hashset;
use im_rc::HashMap;
use im_rc::HashSet;
use tailcall::tailcall;
use itertools::Itertools;
use std::fmt::Display;
use ansi_term::Color;
use std::str::FromStr;
use utils::input::*;
use utils::output::*;
use utils::results::*;

#[macro_use]
extern crate log;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let day = parse_day(file!());
    let file = file_name_from_args();
    let map = parse_string_from_file(&path_for_day(day, &file).unwrap()).unwrap();

    info!("Solution to part one: {}", display_result(part_one(&map)));
    info!("Solution to part two: {}", display_result(part_two(&map)));
}

fn part_one(map: &HeightMap) -> Result<usize> {
    Ok(map.minima().map(|(_, h)| h + 1).sum())
}

fn part_two(map: &HeightMap) -> Result<usize> {
    
    #[tailcall]
    fn calculate_basin( map: &HeightMap, seeds: HashSet<(usize,usize)>, basin_positions: HashSet<(usize,usize)> ) -> HashSet<(usize,usize)> {
        match seeds.iter().next() {
            None => basin_positions,
            Some( seed ) => {
                let next_points: HashSet<(usize,usize)> = map.neighbours_of( &seed.0, &seed.1 ).iter()
                    .filter( |&(_,h)| *h < 9 )
                    .map( |(p,_)| p )
                    .filter( |pos| !basin_positions.contains( pos ) )
                    .copied()
                    .collect();

                calculate_basin( map, seeds.without( seed ).union( next_points ), basin_positions.update( *seed ) )
            }
        }
    }

    let basins = map.minima().map( |(pos,_)| calculate_basin( map, hashset![ pos ], HashSet::new() ) ).collect();

    colorize( map, &basins );
    let product = basins.iter().map( |b| b.len() ).sorted_by_key( |&s| -1 * s as i64 ).take( 3 ).product();
    Ok( product )
}

fn colorize( map: &HeightMap, basins: &Vec<HashSet<(usize,usize)>> ) {
    let colours = vec![
        Color::Blue, Color::Cyan, Color::Green, Color::Purple, Color::Red, Color::White, Color::Yellow
    ];

    let color_map: HashMap<(usize,usize),Color> = basins.iter().zip( colours.into_iter().cycle() ).flat_map( |(basin,color)| basin.into_iter().map( move |&pos| (pos,color) ) ).collect();
    let minima: HashSet<(usize,usize)> = map.minima().map( |(pos,_)| pos ).collect();

    let border = ansi_term::Style::new().bold().on( Color::Black );

    for y in 0 .. map.height {
        for x in 0 .. map.width {
            let v = map.height_at( &x, &y ).unwrap();
            let style = color_map.get( &(x,y) ).map( |&color| {
                if minima.contains( &(x,y) ) {
                    ansi_term::Style::new().fg( color ).bold()
                } else {
                    ansi_term::Style::new().fg( color ).dimmed()
                }
            } ).unwrap_or( border );

            print!( "{}", style.paint( format!( "{}", v ) ) );
        }
        println!("")
    }
}

#[derive(Debug)]
struct HeightMap {
    heights: HashMap<(usize, usize), usize>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn new(input: &Vec<Vec<usize>>) -> HeightMap {
        let heights = input
            .iter()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, &height)| ((x, y), height))
            })
            .collect();

        let width = input.first().map(|r| r.len()).unwrap_or(0);
        let height = input.len();

        HeightMap {
            heights,
            width,
            height,
        }
    }

    fn height_at(&self, x: &usize, y: &usize) -> Option<&usize> {
        self.heights.get(&(*x, *y))
    }

    fn neighbours_of(&self, x: &usize, y: &usize) -> HashMap<(usize, usize), usize> {
        let positions = hashset![
            (*x as isize, *y as isize - 1),
            (*x as isize, *y as isize + 1),
            (*x as isize + 1, *y as isize),
            (*x as isize - 1, *y as isize)
        ];
        positions
            .iter()
            .filter_map(|&(x, y)| {
                if x >= 0 && y >= 0 {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            })
            .flat_map(|(x, y)| self.height_at(&x, &y).map(|h| ((x, y), *h)))
            .collect()
    }

    fn positions(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.heights.keys()
    }

    fn minima(&self) -> impl Iterator<Item = ((usize, usize), usize)> + '_ {
        self.positions().filter_map(|&(x, y)| {
            self.height_at(&x, &y)
                .filter(|height| {
                    self.neighbours_of(&x, &y)
                        .iter()
                        .all(|(_, n_height)| n_height > height)
                })
                .map(|&h| ((x, y), h))
        })
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let map = (0..self.height)
            .map(|y| {
                let line: String = (0..self.width)
                    .map(|x| format!("{}", self.heights.get(&(x, y)).unwrap()))
                    .join("");
                line
            })
            .join("\n");

        write!(f, "{}", map)
    }
}

impl FromStr for HeightMap {
    type Err = AOCError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        input
            .lines()
            .map(|l| {
                let line_heights: Result<Vec<usize>> = l
                    .chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|i| i as usize)
                            .ok_or(AOCError::new_from_ref("Invalid char"))
                    })
                    .try_collect();
                line_heights
            })
            .try_collect()
            .map(|heights| HeightMap::new(&heights))
    }
}
