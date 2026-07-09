use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashSet},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let ans = a_star(State::start(), State::target()).context("no path found")?;
    println!("{}", ans);

    // let ans = djikstra(State::start_part2(), State::target_part2()).context("no path found")?;
    // println!("{}", ans);

    Ok(())
}

/// (Shouldn't really be Ord, but we're using it in a BinaryHeap.)
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    elevator: usize,
    floors: [BTreeSet<Object>; 4],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Object {
    element: String,
    type_: ObjectType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ObjectType {
    Microchip,
    Generator,
}

/// Find the shortest path from source to target.
fn a_star(source: State, target: State) -> Option<usize> {
    let mut to_visit = BinaryHeap::new(); // min-heap
    let mut visited = HashSet::new();

    let t = source.target_estimate();
    to_visit.push((Reverse(0 + t), 0, t, source));

    while let Some((_, source_est, _, curr)) = to_visit.pop() {
        if visited.contains(&curr) {
            continue;
        }
        visited.insert(curr.clone());

        if curr == target {
            dbg!(visited.len());
            return Some(source_est);
        }

        for next in curr.neighbors() {
            let s = source_est + 1;
            let t = next.target_estimate();
            to_visit.push((Reverse(s + t), s, t, next));
        }
    }

    None
}

impl State {
    /// This must be a lower bound on the distance to the target.
    fn target_estimate(&self) -> usize {
        let mut total_cost = 0;
        let mut num_objects = 0;
        for (i, f) in self.floors.iter().enumerate() {
            num_objects += f.len();
            total_cost += if num_objects >= 2 {
                num_objects * 2 - 3
            } else {
                1
            };
        }
        total_cost / 2
    }

    fn neighbors(&self) -> Vec<Self> {
        self.neighbors_including_invalid()
            .into_iter()
            .filter(Self::is_valid)
            .collect()
    }

    fn neighbors_including_invalid(&self) -> Vec<Self> {
        // can go up or down
        // can take any 1, or any 2 items with you

        let mut out = vec![];

        for dirn in [-1, 1] {
            let old_floor = self.elevator;
            let new_floor = isize::try_from(old_floor).unwrap() + dirn;

            let in_bounds =
                0 <= new_floor && new_floor < isize::try_from(self.floors.len()).unwrap();
            if !in_bounds {
                continue;
            }
            let new_floor = usize::try_from(new_floor).unwrap();

            // Note that x and y can be the same.
            for x in &self.floors[old_floor] {
                for y in &self.floors[old_floor] {
                    let mut new_state = self.clone();
                    new_state.elevator = new_floor;
                    new_state.floors[old_floor].remove(x);
                    new_state.floors[old_floor].remove(y);
                    new_state.floors[new_floor].insert(x.clone());
                    new_state.floors[new_floor].insert(y.clone());
                    out.push(new_state);
                }
            }
        }

        out
    }

    fn is_valid(&self) -> bool {
        assert!(self.elevator < self.floors.len());
        self.floors.iter().all(|floor| {
            floor.iter().all(|x| {
                let generator = x.type_ == ObjectType::Generator;
                let protected = floor
                    .iter()
                    .any(|y| x.element == y.element && y.type_ == ObjectType::Generator);
                let fried = floor.iter().any(|y| y.type_ == ObjectType::Generator);
                let safe = generator || protected || !fried;
                safe
            })
        })
    }
}

/// Skip input parsing; hard-code instead.
impl State {
    fn start() -> Self {
        Self {
            elevator: 0,
            floors: [
                vec![
                    Object {
                        element: "promethium".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "promethium".into(),
                        type_: ObjectType::Microchip,
                    },
                ],
                vec![
                    Object {
                        element: "cobalt".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "curium".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "ruthenium".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "plutonium".into(),
                        type_: ObjectType::Generator,
                    },
                ],
                vec![
                    Object {
                        element: "cobalt".into(),
                        type_: ObjectType::Microchip,
                    },
                    Object {
                        element: "curium".into(),
                        type_: ObjectType::Microchip,
                    },
                    Object {
                        element: "ruthenium".into(),
                        type_: ObjectType::Microchip,
                    },
                    Object {
                        element: "plutonium".into(),
                        type_: ObjectType::Microchip,
                    },
                ],
                vec![],
            ]
            .map(|vec| vec.into_iter().collect()),
        }
    }

    fn target() -> Self {
        Self {
            elevator: 3,
            floors: [
                vec![],
                vec![],
                vec![],
                vec![
                    Object {
                        element: "promethium".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "promethium".into(),
                        type_: ObjectType::Microchip,
                    },
                    Object {
                        element: "cobalt".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "curium".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "ruthenium".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "plutonium".into(),
                        type_: ObjectType::Generator,
                    },
                    Object {
                        element: "cobalt".into(),
                        type_: ObjectType::Microchip,
                    },
                    Object {
                        element: "curium".into(),
                        type_: ObjectType::Microchip,
                    },
                    Object {
                        element: "ruthenium".into(),
                        type_: ObjectType::Microchip,
                    },
                    Object {
                        element: "plutonium".into(),
                        type_: ObjectType::Microchip,
                    },
                ],
            ]
            .map(|vec| vec.into_iter().collect()),
        }
    }

    fn start_part2() -> Self {
        let mut state = Self::start();
        for obj in part2_objects() {
            state.floors[0].insert(obj);
        }
        state
    }

    fn target_part2() -> Self {
        let mut state = Self::target();
        for obj in part2_objects() {
            state.floors[0].insert(obj);
        }
        state
    }
}

fn part2_objects() -> Vec<Object> {
    vec![
        Object {
            element: "elerium".into(),
            type_: ObjectType::Generator,
        },
        Object {
            element: "elerium".into(),
            type_: ObjectType::Microchip,
        },
        Object {
            element: "dilithium".into(),
            type_: ObjectType::Generator,
        },
        Object {
            element: "dilithium".into(),
            type_: ObjectType::Microchip,
        },
    ]
}
