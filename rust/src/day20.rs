use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    process::Output,
};

type Value = bool;
type Name = String;
type Outputs = Vec<String>;
type Inputs = HashMap<Name, Value>;

#[derive(Debug)]
enum Module {
    Broadcast {
        outputs: Outputs,
    },
    FlipFlop {
        name: Name,
        value: Value,
        outputs: Outputs,
    },
    Con {
        name: Name,
        inputs: Inputs,
        outputs: Outputs,
    },
}
type Modules = Vec<Module>;
type ModulesAddr = HashMap<String, usize>;

#[derive(Clone, Debug)]
struct Pulse {
    src: Name,
    dst: Name,
    lvl: Value,
}
impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -{}-> {}",
            self.src,
            if self.lvl { "high" } else { "low" },
            self.dst
        )
    }
}
type Pulses = VecDeque<Pulse>; // Stack

impl Module {
    fn from(line: &str) -> Module {
        let mut it = line.split_terminator(" -> ");
        let type_and_name = it.next().unwrap();
        let outputs: Vec<String> = it
            .next()
            .unwrap()
            .split(',')
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.trim().to_string())
            .collect();

        if type_and_name == "broadcaster" {
            return Module::Broadcast { outputs };
        } else if type_and_name.as_bytes()[0] == b'%' {
            return Module::FlipFlop {
                name: type_and_name[1..].to_string(),
                value: false,
                outputs,
            };
        } else {
            return Module::Con {
                name: type_and_name[1..].to_string(),
                inputs: Inputs::new(),
                outputs,
            };
        }
    }

    fn all_from(input: &str) -> Modules {
        let mut modules = Modules::new();
        for line in input.lines() {
            modules.push(Module::from(line));
        }

        // add inputs to Con modules
        let inputs: Vec<_> = modules
            .iter()
            .flat_map(|m| {
                let (src, dsts) = match m {
                    Module::Broadcast { outputs } => ("broadcaster", outputs),
                    Module::FlipFlop {
                        name,
                        value: _,
                        outputs,
                    } => (name.as_str(), outputs),
                    Module::Con {
                        name,
                        inputs: _,
                        outputs,
                    } => (name.as_str(), outputs),
                };
                dsts.iter().map(|dst| (dst.to_string(), src.to_string()))
            })
            .collect();

        for m in modules.iter_mut() {
            match m {
                Module::Broadcast { outputs: _ } => {}
                Module::FlipFlop {
                    name: _,
                    value: _,
                    outputs: _,
                } => {}
                Module::Con {
                    name,
                    inputs: module_inputs,
                    outputs: _,
                } => inputs
                    .iter()
                    .filter(|(idst, _)| idst == name)
                    .map(|(_, isrc)| isrc)
                    .for_each(|src| {
                        module_inputs.insert(src.to_string(), false);
                    }),
            }
        }

        modules
    }
}

fn process(modules: &mut Modules, pulse: &Pulse, addr: &ModulesAddr) -> Pulses {
    let idx = addr.get(&pulse.dst);
    if (idx.is_none()) {
        return Pulses::new();
    }

    let mut dst_mod = &mut modules[*idx.unwrap()];
    /*
    .iter_mut().find(|module| match module {
        Module::Broadcast { outputs } => return pulse.dst == "broadcaster",
        Module::FlipFlop {
            name,
            value,
            outputs,
        } => return &pulse.dst == name,
        Module::Con {
            name,
            inputs,
            outputs,
        } => return &pulse.dst == name,
    });

    if dst_mod.is_none() {
    }
    */

    match dst_mod {
        Module::Broadcast { outputs } => outputs
            .iter()
            .map(|output| Pulse {
                src: "broadcaster".to_string(),
                dst: output.to_string(),
                lvl: false,
            })
            .collect(),
        Module::FlipFlop {
            name,
            value,
            outputs,
        } => {
            if !pulse.lvl {
                *value = if *value { false } else { true };
                outputs
                    .iter()
                    .map(|output| Pulse {
                        src: name.to_string(),
                        dst: output.to_string(),
                        lvl: *value,
                    })
                    .collect()
            } else {
                Pulses::new()
            }
        }
        Module::Con {
            name,
            inputs,
            outputs,
        } => {
            *inputs.get_mut(&pulse.src).unwrap() = pulse.lvl;
            let lvl = inputs.iter().filter(|(_, &lvl)| lvl == false).count() > 0;
            outputs
                .iter()
                .map(|output| Pulse {
                    src: name.to_string(),
                    dst: output.to_string(),
                    lvl,
                })
                .collect()
        }
    }
}

pub fn task1(input: &str) -> i64 {
    let mut modules = Module::all_from(input);
    let mut modules_addr = ModulesAddr::new();
    modules.iter().enumerate().for_each(|(i, m)| {
        let name = match m {
            Module::Broadcast { outputs: _ } => "broadcaster".to_string(),
            Module::FlipFlop {
                name,
                value: _,
                outputs: _,
            } => name.to_string(),
            Module::Con {
                name,
                inputs: _,
                outputs: _,
            } => name.to_string(),
        };
        modules_addr.insert(name, i);
    });

    let mut pulses_all = Pulses::new();
    let mut pulses_act = Pulses::new();

    for _i in 0..1000 {
        pulses_act.push_back(Pulse {
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
            lvl: false,
        });

        while !pulses_act.is_empty() {
            let pulse = pulses_act.pop_back().unwrap();
            //       println!("{:?}", modules);
            pulses_all.push_back(pulse.clone());
            //println!("{}", pulse);
            let new_pulses = process(&mut modules, &pulse, &modules_addr);
            new_pulses.iter().for_each(|pulse| {
                //               pulses_all.push_back(pulse.clone());
                pulses_act.push_front(pulse.clone());
            });
        }
    }

    let pos = pulses_all.iter().filter(|p| p.lvl).count() as i64;
    let neg = pulses_all.len() as i64 - pos as i64;
    //println!("p:{} n:{} = {}", pos, neg, pos * neg);

    //println!("{:?}", modules);
    return pos * neg;
}

pub fn task2(input: &str) -> i64 {
    let mut modules = Module::all_from(input);
    let mut modules_addr = ModulesAddr::new();
    modules.iter().enumerate().for_each(|(i, m)| {
        let name = match m {
            Module::Broadcast { outputs: _ } => "broadcaster".to_string(),
            Module::FlipFlop {
                name,
                value: _,
                outputs: _,
            } => name.to_string(),
            Module::Con {
                name,
                inputs: _,
                outputs: _,
            } => name.to_string(),
        };
        modules_addr.insert(name, i);
    });
    let mut pulses_act = Pulses::new();

    for i in 0.. {
        let mut pulses_all = Pulses::new();
        pulses_act.push_back(Pulse {
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
            lvl: false,
        });

        while !pulses_act.is_empty() {
            let pulse = pulses_act.pop_back().unwrap();
            //       println!("{:?}", modules);
            pulses_all.push_back(pulse.clone());
            //println!("{}", pulse);
            let new_pulses = process(&mut modules, &pulse, &modules_addr);
            new_pulses.iter().for_each(|pulse| {
                //               pulses_all.push_back(pulse.clone());
                pulses_act.push_front(pulse.clone());
            });
        }
        let count = pulses_all.iter().filter(|p| p.dst == "rx").count();
        //println!("#{}: {}", i, count);
        if count == 1 {
            return i;
        }
        //if i == 1000000 {
        //    return -1;
        //}
    }
    return 0;
}

#[test]
fn task1_test() {
    let data = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;
    assert_eq!(task1(data), 32000000);
}

#[test]
fn task1_test2() {
    let data = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;
    assert_eq!(task1(data), 11687500);
}
