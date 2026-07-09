use std::collections::{BTreeSet, HashSet, VecDeque};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let ans = bfs(State::start(), State::target()).context("no path found")?;
    println!("{}", ans);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// Find the shortest path from start to target.
fn bfs(start: State, target: State) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();

    seen.insert(start.clone());
    to_visit.push_back((start, 0));

    while let Some((curr, dist)) = to_visit.pop_front() {
        if curr == target {
            return Some(dist);
        }

        for next in curr.neighbors() {
            if !seen.contains(&next) {
                seen.insert(next.clone());
                to_visit.push_back((next, dist + 1));
            }
        }
    }

    None
}

impl State {
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
}
