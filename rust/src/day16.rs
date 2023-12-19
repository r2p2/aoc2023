#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display};

#[derive(Hash, Eq, PartialEq, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Dir::Up => "u",
            Dir::Down => "d",
            Dir::Left => "l",
            Dir::Right => "r",
        };
        write!(f, "{}", s)
    }
}
#[derive(Hash, Eq, PartialEq, Clone)]
struct Beam {
    idx: usize,
    dir: Dir,
}

fn printbeams(beams: &Vec<Beam>, width: usize) {
    println!("");
    for beam in beams {
        println!(
            "- {}({},{}) {}",
            beam.idx,
            beam.idx % width,
            beam.idx / width,
            beam.dir
        );
    }
    println!("---");
}

fn process(input: &str, init_beam: Beam) -> usize {
    let width = input.find('\n').unwrap() + 1;
    let mut energized: HashSet<usize> = HashSet::new();
    let mut beams_prev: HashSet<Beam> = HashSet::new();
    let mut beams: Vec<Beam> = vec![init_beam];

    while beams.len() > 0 {
        let mut new_beams: Vec<Beam> = Vec::new();
        {
            // process new directions
            let mut i = 0;
            while i < beams.len() {
                let beam = &mut beams[i];

                // keep beam positions in mind
                let mut remove = !beams_prev.insert(beam.clone());
                energized.insert(beam.idx);

                if !remove {
                    match input.chars().nth(beam.idx) {
                        // FIXME(RPE): nth() is O(n)
                        Some('|') => {
                            if beam.dir == Dir::Left || beam.dir == Dir::Right {
                                remove = true;
                                new_beams.push(Beam {
                                    idx: beam.idx,
                                    dir: Dir::Up,
                                });
                                new_beams.push(Beam {
                                    idx: beam.idx,
                                    dir: Dir::Down,
                                });
                            }
                        }
                        Some('-') => {
                            if beam.dir == Dir::Up || beam.dir == Dir::Down {
                                remove = true;
                                new_beams.push(Beam {
                                    idx: beam.idx,
                                    dir: Dir::Left,
                                });
                                new_beams.push(Beam {
                                    idx: beam.idx,
                                    dir: Dir::Right,
                                });
                            }
                        }
                        Some('\\') => {
                            beam.dir = match beam.dir {
                                Dir::Up => Dir::Left,
                                Dir::Down => Dir::Right,
                                Dir::Left => Dir::Up,
                                Dir::Right => Dir::Down,
                            };
                        }
                        Some('/') => {
                            beam.dir = match beam.dir {
                                Dir::Up => Dir::Right,
                                Dir::Down => Dir::Left,
                                Dir::Left => Dir::Down,
                                Dir::Right => Dir::Up,
                            };
                        }
                        _ => {}
                    }
                }

                if remove {
                    beams.remove(i);
                    continue;
                }

                i += 1;
            }
        }
        beams.append(&mut new_beams);
        {
            // move beams forward
            let mut i = 0;
            while i < beams.len() {
                let beam = &mut beams[i];
                let new_idx = match beam.dir {
                    Dir::Up => {
                        if beam.idx > width {
                            Some(beam.idx - width)
                        } else {
                            None
                        }
                    }
                    Dir::Down => {
                        if beam.idx < (input.len() - width) {
                            Some(beam.idx + width)
                        } else {
                            None
                        }
                    }
                    Dir::Left => {
                        if (beam.idx % width) != 0 {
                            Some(beam.idx - 1)
                        } else {
                            None
                        }
                    }
                    Dir::Right => {
                        if (beam.idx % width) != (width - 2) {
                            Some(beam.idx + 1)
                        } else {
                            None
                        }
                    }
                };

                if let Some(idx) = new_idx {
                    beam.idx = idx;
                } else {
                    beams.remove(i);
                    continue;
                }
                i += 1;
            }
        }
        //printbeams(&beams, width);
    }

    return energized.len();

}

pub fn task1(input: &str) -> usize {
    process(input, Beam {
        idx: 0,
        dir: Dir::Right,
    })
}

pub fn task2(input: &str) -> usize {
    let mut max = 0;

    let width = input.find('\n').unwrap()+1;
    for x in 0..(width) {
        max = std::cmp::max(max, process(input, Beam {idx: x, dir: Dir::Down}));
        max = std::cmp::max(max, process(input, Beam {idx: input.len()-2-x, dir: Dir::Up}));
    }
    let mut y = 0;
    while y < input.len() {
        max = std::cmp::max(max, process(input, Beam {idx: y, dir: Dir::Right}));
        max = std::cmp::max(max, process(input, Beam {idx: y+width-2, dir: Dir::Left}));
        y += width;
    }

    return max;
}

#[test]
fn task0_test() {
    let data = r#".........\
/......../
\...-....\
/...---../
\........\
/......../
\........\
/......../
\........\
/......../
"#;
    assert_eq!(task1(data), 100);
}

#[test]
fn task1_test() {
    let data = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
    assert_eq!(task1(data), 46);
}

#[test]
fn task2_test() {
    let data = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
    assert_eq!(task2(data), 51);
}
