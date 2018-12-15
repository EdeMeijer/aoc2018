//! Solutions for https://adventofcode.com/2018/day/15
use std;

use days::day15::Tile::ActorRef;
use utils::matrix::Matrix;
use std::collections::HashSet;
use std::mem::swap;
use utils::data::non_empty_lines;
use utils::data::load_data;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

#[allow(dead_code)]
pub fn part1() {
    println!("{}", solve_part1(get_puzzle_input()));
}

#[allow(dead_code)]
pub fn part2() {
    println!("{}", solve_part2(get_puzzle_input()));
}

const ADJACENT_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
type Grid = Matrix<Tile>;

#[derive(Clone)]
struct World {
    grid: Grid,
    actors: Vec<Actor>,
    elf_ap: i16
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    ActorRef(usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let s = match self {
            Tile::Empty => ".".to_owned(),
            Tile::Wall => "#".to_owned(),
            Tile::ActorRef(i) => i.to_string()
        };
        f.write_str(&s)
    }
}

#[derive(Debug, Clone)]
struct Actor {
    clan: Clan,
    loc: (usize, usize),
    hp: i16,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Clan {
    Elf,
    Goblin,
}

struct BattleResult {
    checksum: u32,
    dead_elfs: usize
}

fn solve_part1(world: World) -> u32 {
    perform_battle(world).checksum
}

fn solve_part2(mut world: World) -> u32 {
    loop {
        let result = perform_battle( world.clone());
        
        if result.dead_elfs == 0 {
            break result.checksum
        }
        
        world.elf_ap += 1;
    }
}

fn perform_battle(mut world: World) -> BattleResult {
    let mut full_rounds = 0;
    loop {
        let result = perform_round(world);
        world = result.0;

        if result.1 {
            // Battle ended
            break;
        }
        full_rounds += 1;
    }

    let remaining_hp = world.actors.iter()
        .map(|a| a.hp.max(0) as u32)
        .sum::<u32>();
    
    let dead_elfs = world.actors.iter()
        .filter(|a| a.clan == Clan::Elf)
        .filter(|a| a.hp <= 0)
        .count();

    BattleResult {
        checksum: remaining_hp * full_rounds,
        dead_elfs
    }
}

fn shift_loc(loc: (usize, usize), shift: (i32, i32)) -> (usize, usize) {
    ((loc.0 as i32 + shift.0) as usize, (loc.1 as i32 + shift.1) as usize)
}

fn perform_round(world: World) -> (World, bool) {
    let World { mut grid, mut actors, elf_ap } = world;
    
    // Determine the actor ordering
    let mut actor_order: Vec<_> = actors.iter().enumerate()
        .map(|(i, a)| (i, a.loc))
        .collect();
    
    actor_order.sort_by_key(|tup| tup.1);
    let actor_order: Vec<_> = actor_order.into_iter().map(|tup| tup.0).collect();
    
    let mut battle_ended = false;

    // Let every actor perform their actions, if not dead
    for i in actor_order.into_iter() {
        if actors[i].hp > 0 {
            // Check if there's any enemies remaining at all, if not the battle has ended
            let mut any_enemies = false;
            for j in 0..actors.len() {
                if actors[j].clan != actors[i].clan && actors[j].hp > 0 {
                    any_enemies = true;
                    break;
                }
            }
            if !any_enemies {
                battle_ended = true;
                break;
            }
            
            let mut enemies_in_range = get_enemies_in_range(&actors, i, &grid);
            
            if enemies_in_range.is_empty() {
                // No enemy in range, so we need to determine the best step, if any
                let mut distances = Matrix::new(grid.height, grid.width, std::u32::MAX);
                let mut initial_dirs = Matrix::new(grid.height, grid.width, 4usize);
                for (o, offset) in ADJACENT_OFFSETS.iter().enumerate() {
                    initial_dirs[shift_loc(actors[i].loc, *offset)] = o;
                }
                distances[actors[i].loc] = 0;
                let mut frontier = vec![actors[i].loc];

                let mut dir_to_target = None;

                while dir_to_target.is_none() && !frontier.is_empty() {
                    // For every entry in the frontier, find cells that have a higher distance than
                    // the next distance. 
                    let mut next_frontier = HashSet::new();
                    
                    for origin in &frontier {
                        let next_dist = distances[*origin] + 1;
                        let cur_dir = initial_dirs[*origin];

                        for offset in &ADJACENT_OFFSETS {
                            let loc = shift_loc(*origin, *offset);

                            match grid[loc] {
                                Tile::Empty => {
                                    if distances[loc] > next_dist || (distances[loc] == next_dist && cur_dir < initial_dirs[loc]) {
                                        // We found a faster way to get here, or a way that starts
                                        // from a favorable initial move.
                                        distances[loc] = next_dist;
                                        initial_dirs[loc] = initial_dirs[loc].min(cur_dir);
                                        next_frontier.insert(loc);
                                    }
                                }
                                Tile::ActorRef(a) => {
                                    if actors[a].clan != actors[i].clan {
                                        dir_to_target = match dir_to_target {
                                            None => Some(cur_dir),
                                            Some(old) => Some(old.min(cur_dir))
                                        };
                                    }
                                },
                                Tile::Wall => {}
                            }
                        }
                    }
                    
                    frontier = next_frontier.into_iter().collect();
                }
                
                if let Some(dir) = dir_to_target {
                    // Found a way to get to a target, let's move!
                    let next_loc = shift_loc(actors[i].loc, ADJACENT_OFFSETS[dir]);
                    let mut actor_ref = Tile::Empty;
                    swap(&mut grid[actors[i].loc], &mut actor_ref);
                    grid[next_loc] = actor_ref;

                    actors[i].loc = next_loc;
                    
                    // Update the enemies in range
                    enemies_in_range = get_enemies_in_range(&actors, i, &grid);
                }
            }
            
            if !enemies_in_range.is_empty() {
                // Fight!
                // Filter the enemies in range that have the lowest hp
                let min_hp = enemies_in_range.iter()
                    .map(|a| actors[*a].hp)
                    .min().unwrap();
                
                enemies_in_range = enemies_in_range.into_iter()
                    .filter(|a| actors[*a].hp == min_hp)
                    .collect();
                
                // Fight the first enemy (already sorted by reading order)
                let enemy_i = enemies_in_range[0];
                
                let ap = if actors[i].clan == Clan::Elf { elf_ap } else { 3 };
                actors[enemy_i].hp -= ap;
                
                if actors[enemy_i].hp <= 0 {
                    // He ded, remove from grid
                    grid[actors[enemy_i].loc] = Tile::Empty;
                }
            }
        }
    }

    (World { grid, actors, elf_ap }, battle_ended)
}

fn get_enemies_in_range(actors: &Vec<Actor>, actor_i: usize, grid: &Grid) -> Vec<(usize)> {
    let actor = &actors[actor_i];
    let mut result = vec![];
    
    for offset in &ADJACENT_OFFSETS {
        let loc = shift_loc(actor.loc, *offset);
        if let ActorRef(a) = grid[loc] {
            if actors[a].clan != actor.clan {
                result.push(a);
            }
        }
    }
    result
}

fn get_puzzle_input() -> World {
    parse_input(load_data("day15"))
}

fn parse_input(input: String) -> World {
    let lines = non_empty_lines(input);
    let height = lines.len();
    let width = lines[0].len();
    
    let mut grid = Grid::new(height, width, Tile::Empty);
    let mut actors = vec![];
    
    for (y, line) in lines.into_iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let loc = (y, x);
            let tile = match cell { 
                '#' => Tile::Wall,
                'E'|'G' => {
                    let a = actors.len();
                    let clan = if cell == 'E' { Clan::Elf } else { Clan::Goblin };
                    actors.push(Actor { clan, loc, hp: 200 });
                    ActorRef(a)
                }
                _ => Tile::Empty
            };
            grid[loc] = tile;
        }
    }
    
    World { grid, actors, elf_ap: 3 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = String::from(r"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
");
        
        assert_eq!(
            solve_part1( parse_input(input)),
            27730
        );
    }

    #[test]
    fn test_part2() {
        let input = String::from(r"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
");

        assert_eq!(
            solve_part2( parse_input(input)),
            4988
        );
    }
}
