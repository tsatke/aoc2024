use ahash::AHashSet;
use std::ops::{Index, IndexMut};

const LENGTH: usize = 130;
const INPUT: &str = include_str!("../inputs/input_day6.txt");

struct Grid([[Tile; LENGTH]; LENGTH]);

impl Index<Coord> for Grid {
    type Output = Tile;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

impl IndexMut<Coord> for Grid {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Free,
    Visited,
    Obstacle,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, coord: Coord) -> Option<Coord> {
        match self {
            Direction::Up => coord.up(),
            Direction::Down => coord.down(),
            Direction::Left => coord.left(),
            Direction::Right => coord.right(),
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord(u8, u8);

impl Coord {
    fn up(self) -> Option<Self> {
        if self.0 == 0 {
            return None;
        }
        Some(Self(self.0 - 1, self.1))
    }

    fn down(self) -> Option<Self> {
        if self.0 == LENGTH as u8 - 1 {
            return None;
        }
        Some(Self(self.0 + 1, self.1))
    }

    fn left(self) -> Option<Self> {
        if self.1 == 0 {
            return None;
        }
        Some(Self(self.0, self.1 - 1))
    }

    fn right(self) -> Option<Self> {
        if self.1 == LENGTH as u8 - 1 {
            return None;
        }
        Some(Self(self.0, self.1 + 1))
    }
}

fn parse_input() -> (Coord, Grid) {
    let mut pos = Coord(0, 0);
    let mut grid = [[Tile::Free; LENGTH]; LENGTH];
    grid.iter_mut()
        .enumerate()
        .zip(INPUT.lines())
        .for_each(|((row_num, row), line)| {
            row.iter_mut()
                .enumerate()
                .zip(line.as_bytes().iter())
                .for_each(|((col_num, tile), &b)| {
                    *tile = match b {
                        b'.' => Tile::Free,
                        b'#' => Tile::Obstacle,
                        b'^' => {
                            pos = Coord(row_num as u8, col_num as u8);
                            Tile::Visited
                        }
                        _ => unreachable!(),
                    };
                });
        });
    (pos, Grid(grid))
}

#[must_use]
pub fn part1() -> usize {
    let (mut pos, mut grid) = parse_input();
    let mut direction = Direction::Up;

    let mut count = 1;
    loop {
        let next_pos = direction.apply(pos);
        if next_pos.is_none() {
            break;
        }
        let next_pos = next_pos.unwrap();
        let next_tile = grid[next_pos];
        if next_tile == Tile::Obstacle {
            direction = direction.rotate_right();
        } else {
            if next_tile != Tile::Visited {
                count += 1;
            }
            pos = next_pos;
            grid[pos] = Tile::Visited;
        }
    }

    count
}

#[must_use]
pub fn part2() -> usize {
    let (mut pos, mut grid) = parse_input();
    let mut direction = Direction::Up;

    let mut visited = AHashSet::new();
    let mut ghost_visited = AHashSet::new();

    let mut count = 0;
    loop {
        let Some(pos_ahead) = direction.apply(pos) else {
            break;
        };
        let tile = grid[pos_ahead];
        if tile == Tile::Obstacle {
            visited.insert((pos, direction));
            direction = direction.rotate_right();
            continue;
        }

        // place ghost obstacle
        let obstacle_pos = pos_ahead;
        if grid[obstacle_pos] == Tile::Free {
            grid[obstacle_pos] = Tile::Obstacle;

            // tile ahead is not an obstacle, so before stepping, check whether turning to the right here would force a loop
            {
                ghost_visited.clear();
                let mut ghost_pos = pos;
                let mut ghost_direction = direction.rotate_right();

                'ghost_walk: while let Some(ghost_pos_ahead) = ghost_direction.apply(ghost_pos) {
                    if grid[ghost_pos_ahead] == Tile::Obstacle {
                        if (grid[ghost_pos] == Tile::Visited
                            && visited.contains(&(ghost_pos, ghost_direction)))
                            || ghost_visited.contains(&(ghost_pos, ghost_direction))
                        {
                            count += 1;
                            break 'ghost_walk;
                        }
                        ghost_visited.insert((ghost_pos, ghost_direction));
                        ghost_direction = ghost_direction.rotate_right();
                    } else {
                        ghost_pos = ghost_pos_ahead;
                    }
                }
            }
            // reset the ghost obstacle
            grid[obstacle_pos] = Tile::Free;
        }

        // loop check done, step as usual
        pos = pos_ahead;
        grid[pos] = Tile::Visited;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 5305);
        assert_eq!(part2(), 2143);
    }
}
