extern crate rand;
extern crate termion;
use std::{thread, time, env};
use std::fs::File;
use std::io::{BufRead,BufReader};
use termion::{color, clear};

fn census(_world: [[u8;75];75]) -> u16 {
    let mut count = 0;
    for i in 0..74 {
        for j in 0..74 {
            if _world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}

fn generation(_world: [[u8;75];75]) -> [[u8;75];75] {
    let mut newworld = [[0u8;75];75];

    for i in 0..74 {
        for j in 0..74 {
            let mut count = 0;
            if i > 0 {
                count += _world[i-1][j];
            }
            else if i > 0 && j > 0 {
                count += _world[i-1][j-1];
            }
            else if j > 0 {
                count += _world[i][j-1];
            }
            else if i > 0 && j < 74 {
                count += _world[i-1][j+1];
            }
            else if i < 74 {
                count += _world[i+1][j];
            }
            else if i < 74 && j > 0 {
                count += _world[i+1][j-1];
            }
            else if j < 74 {
                count += _world[i][j+1];
            }
            else if i < 74 && j < 74 {
                count += _world[i+1][j+1];
            }
            newworld[i][j] = 0;
            if _world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            } else if _world[i][j] == 0 && count == 3 {
                newworld[i][j] = 1;
            }
        }
    }

    newworld
}

fn main() {
    let mut world = [[0u8;75];75];
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        for i in 0..74 {
            for j in 0..74 {
                if rand::random() {
                    world[i][j] = 1;
                }else{
                    world[i][j] = 0;
                }
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename); }
        println!("Population at generation {} is {}", generations, census(world)); for _gens in 0..100 {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("{}", clear::All);
        display_world(world);
        println!("{blue}Population at generation {g} is {c}", blue =
        color::Fg(color::Blue), g = generations, c = census(world)); thread::sleep(time::Duration::from_secs(2));        
    }


}

fn display_world(world: [[u8;75];75]) {
    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 {
                print!("{red}*", red = color::Fg(color::Red));
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn populate_from_file(filename: String) -> [[u8;75];75] {
    let mut newworld = [[0u8;75];75];
    
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut pairs : Vec<(usize, usize)> = Vec::new();

    for (_, line) in reader.lines().enumerate(){
        let l: String = line.unwrap();
        let mut words = l.split_whitespace();
        let left =  words.next().unwrap();
        let right = words.next().unwrap();
        pairs.push((left.parse::<usize>().unwrap(),
        right.parse::<usize>().unwrap()));
    }

    for x in 0..74 {
        for y in 0..74 {
            newworld[x][y] = 0;
        }
    }

    for (x,y) in pairs {
        newworld[x][y] = 1;
    }

    newworld
}
