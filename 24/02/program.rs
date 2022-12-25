use std::collections::{HashSet,HashMap};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
  x: usize,
  y: usize,
}

enum Direction {
  Right,
  Down,
  Left,
  Up,
}

impl Coordinate {
  fn add(&mut self, dir: &Direction) {
    match dir {
      Direction::Left => self.x -= 1,
      Direction::Right => self.x += 1,
      Direction::Up => self.y -= 1,
      Direction::Down => self.y += 1,
    }
  }
  
  fn with(&self, dir: &Direction) -> Coordinate {
    let mut clone = self.clone();
    clone.add(dir);
    clone
  }
}

struct Storm (Coordinate, Direction);

impl Storm {
  fn tick(&mut self, max_x: usize, max_y: usize) {
    self.0.add(&self.1);

    if self.0.x < 1 { self.0.x = max_x }
    if self.0.x > max_x { self.0.x = 1 }

    if self.0.y < 1 { self.0.y = max_y }
    if self.0.y > max_y { self.0.y = 1 }
  }
}

struct Valley {
  minute: usize,
  storms: Vec<Storm>,

  max_storm_x: usize,
  max_storm_y: usize,
}

struct Location(Coordinate, usize);

impl Valley {
  fn travel(&mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Result<(), &'static str> {
    let mut locations = vec![Location(Coordinate { x: start_x, y: start_y }, 0)];

    loop {
      if locations.len() == 0 {
        return Err("Failed to find locations");
      }

      println!("{}: {}", self.minute, locations.len());

      self.minute += 1;

      for storm in &mut self.storms {
        storm.tick(self.max_storm_x, self.max_storm_y);
      }

      let storm_locations = (&self.storms).iter().map(|s| s.0).collect::<HashSet<_>>();

      let mut new_locations = HashMap::<Coordinate, usize>::new();
      for Location(coordinate, visits) in locations {
        for new_coordinate in [
          coordinate.clone(),
          coordinate.with(&Direction::Right),
          coordinate.with(&Direction::Down),
          coordinate.with(&Direction::Left),
          coordinate.with(&Direction::Up),
        ] {
          let mut new_visits = visits;
          if new_coordinate.x == end_x && new_coordinate.y == end_y {
            if visits == 0 {
              new_visits = 1;
            } else if visits == 2 {
              return Ok(());
            }
          } else if new_coordinate.x == start_x && new_coordinate.y == start_y {
            if visits == 1 {
              new_visits = 2;
            }
          }

          if storm_locations.contains(&new_coordinate) {
            continue;
          }

          if new_coordinate.y == start_y {
            if new_coordinate.x != start_x {
              continue;
            }
          } else if new_coordinate.y == end_y {
            if new_coordinate.x != end_x {
              continue;
            }
          } else {
            if new_coordinate.x < 1 || new_coordinate.x > self.max_storm_x {
              continue;
            }
            if new_coordinate.y < 1 || new_coordinate.y > self.max_storm_y {
              continue;
            }
          }

          new_locations.entry(new_coordinate).and_modify(|v| {*v = new_visits.max(*v);}).or_insert(new_visits);
        }
      }

      locations = new_locations.into_iter().map(|(k, v)| -> Location {Location(k, v)}).collect();
    }
  }
}

fn main() -> Result<(), &'static str> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("Expected exactly 1 argument, got {}", args.len() - 1);
  }

  if let Ok(_lines) = read_lines(&args[1]) {
    let lines = _lines.collect::<Vec<_>>();
    let mut storms = Vec::<Storm>::new();
    let mut maybe_start_x: Option<usize> = None;
    let mut maybe_max_storm_x: Option<usize> = None;
    let start_y = 0;
    let mut maybe_end_x: Option<usize> = None;
    let end_y = lines.len() - 1;

    for (y, line) in lines.into_iter().enumerate() {
      if y == 0 {
        let line = line.unwrap();
        maybe_max_storm_x = Some(line.len() - 2);
        maybe_start_x = line.chars().position(|c| c == '.');
      } else if y == end_y {
        maybe_end_x = line.unwrap().chars().position(|c| c == '.');
      } else {
        for (x, char) in line.unwrap().chars().enumerate() {
          let coordinate = Coordinate { x, y };
          let direction = match char {
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '^' => Some(Direction::Up),
            _ => None,
          };
  
          if let Some(d) = direction {
            storms.push(Storm (coordinate, d));
          }
        }
      }
    }

    let start_x = maybe_start_x.unwrap();
    let end_x = maybe_end_x.unwrap();

    let max_storm_x = maybe_max_storm_x.unwrap();
    let max_storm_y = end_y - 1;

    let mut valley = Valley {
      minute: 0,
      storms,

      max_storm_x,
      max_storm_y,
    };

    valley.travel(start_x, start_y, end_x, end_y).unwrap();

    println!("{}", valley.minute);
  } else {
    panic!("Failed to read file");
  }

  Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
