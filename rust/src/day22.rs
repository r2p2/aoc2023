use std::collections::HashSet;

use rayon::prelude::*;

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    segments: Vec<Point>,
}
impl Brick {
    fn from(line: &str) -> Brick {
        let mut seg_it = line.split('~');
        let mut begin_it = seg_it.next().unwrap().split(',');
        let mut end_it = seg_it.next().unwrap().split(',');

        let begin = Point {
            x: begin_it.next().unwrap().parse::<usize>().unwrap(),
            y: begin_it.next().unwrap().parse::<usize>().unwrap(),
            z: begin_it.next().unwrap().parse::<usize>().unwrap(),
        };
        let end = Point {
            x: end_it.next().unwrap().parse::<usize>().unwrap(),
            y: end_it.next().unwrap().parse::<usize>().unwrap(),
            z: end_it.next().unwrap().parse::<usize>().unwrap(),
        };
        let start = Point {
            x: begin.x,
            y: begin.y,
            z: begin.z,
        };

        let mut delta = Point {
            x: end.x - begin.x,
            y: end.y - begin.y,
            z: end.z - begin.z,
        };

        let mut segments: Vec<Point> = vec![start];

        while delta.x > 0 || delta.y > 0 || delta.z > 0 {
            segments.push(Point {
                x: begin.x + delta.x,
                y: begin.y + delta.y,
                z: begin.z + delta.z,
            });
            if delta.x > 0 {
                delta.x -= 1;
            } else if delta.y > 0 {
                delta.y -= 1;
            } else if delta.z > 0 {
                delta.z -= 1;
            }
        }

        Brick { id: 0, segments }
    }

    fn is_at_bottom(&self) -> bool {
        return self.segments.iter().find(|s| s.z == 1).is_some();
    }

    fn down(&mut self) {
        if !self.is_at_bottom() {
            self.segments.iter_mut().for_each(|s| s.z -= 1);
        }
    }

    fn copy_one_down(&self) -> Option<Brick> {
        if self.is_at_bottom() {
            return None;
        }
        let mut brick = Brick {
            id: self.id,
            segments: self.segments.clone(),
        };

        brick.segments.iter_mut().for_each(|s| s.z -= 1);

        Some(brick)
    }
}

fn collides(bricks: &Vec<Brick>, other_brick: &Brick) -> bool {
    bricks
        .iter()
        .filter(|b| b.id != other_brick.id)
        .find(|b| {
            b.segments
                .iter()
                .find(|bs| {
                    other_brick
                        .segments
                        .iter()
                        .find(|obs| obs.x == bs.x && obs.y == bs.y && obs.z == bs.z)
                        .is_some()
                })
                .is_some()
        })
        .is_some()
}

fn settle(bricks: &mut Vec<Brick>) -> usize {
    let mut dropped_bricks: HashSet<usize> = HashSet::new();
    loop {
        let mut dropped = false;
        for i in 0..bricks.len() {
            if bricks[i].is_at_bottom() {
                continue;
            }

            if collides(&bricks, &bricks[i].copy_one_down().unwrap()) {
                continue;
            }

            dropped_bricks.insert(bricks[i].id);
            bricks[i].down();
            dropped = true;
        }
        if !dropped {
            return dropped_bricks.len();
        }
    }
}

pub fn task1(input: &str) -> i64 {
    let mut bricks: Vec<_> = Vec::new();
    input
        .lines()
        .for_each(|line| bricks.push(Brick::from(line)));
    let mut id = 0;
    bricks.iter_mut().for_each(|b| {
        b.id = id;
        id += 1;
    });

    settle(&mut bricks);

    bricks
        .par_iter()
        .enumerate()
        .map(|(i, _)| {
            let mut brick_simulation = bricks.clone();
            brick_simulation.swap_remove(i);

            if settle(&mut brick_simulation) == 0 {
                return 1;
            }
            return 0;
        })
        .sum()
}
pub fn task2(input: &str) -> usize {
    let mut bricks: Vec<_> = Vec::new();
    input
        .lines()
        .for_each(|line| bricks.push(Brick::from(line)));
    let mut id = 0;
    bricks.iter_mut().for_each(|b| {
        b.id = id;
        id += 1;
    });

    settle(&mut bricks);

    bricks
        .par_iter()
        .enumerate()
        .map(|(i, _)| {
            let mut brick_simulation = bricks.clone();
            brick_simulation.swap_remove(i);

            return settle(&mut brick_simulation);
        })
        .sum()
}

#[test]
fn task1_test() {
    let data = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;
    assert_eq!(task1(data), 5);
}

#[test]
fn task2_test() {
    let data = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;
    assert_eq!(task2(data), 7);
}
