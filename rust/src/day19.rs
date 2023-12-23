use std::{collections::HashMap, u64};

#[derive(Debug)]
struct Rule {
    condition: bool,
    var: String,
    lt: bool,
    val: u64,
    next: String,
}
struct Workflow {
    rules: Vec<Rule>,
}
impl Workflow {
    fn from(line: &str) -> (String, Workflow) {
        let new_line = line.replace("}", "");
        let mut it = new_line.split('{');
        let name = it.next().unwrap();
        let mut rule_it = it.next().unwrap().split(',');

        let mut wf = Workflow { rules: Vec::new() };

        for rule in rule_it {
            if rule.find(':').is_none() {
                wf.rules.push(Rule {
                    condition: false,
                    var: String::new(),
                    lt: false,
                    val: 0,
                    next: rule.to_string(),
                })
            } else {
                let (rule, next) = rule.split_at(rule.find(':').unwrap());

                wf.rules.push(Rule {
                    condition: true,
                    var: rule.chars().nth(0).unwrap().to_string(),
                    lt: rule.chars().nth(1).unwrap() == '<',
                    val: rule[2..].parse::<u64>().unwrap(),
                    next: next[1..].to_string(),
                })
            }
        }

        (name.to_string(), wf)
    }
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
impl Part {
    fn from(line: &str) -> Part {
        let new_line = line.replace(&['{', '}'][..], "");
        let mut cat_it = new_line.split(',');

        let x_it = cat_it.next().unwrap().split('=');
        let m_it = cat_it.next().unwrap().split('=');
        let a_it = cat_it.next().unwrap().split('=');
        let s_it = cat_it.next().unwrap().split('=');

        Part {
            x: x_it.skip(1).next().unwrap().parse::<u64>().unwrap(),
            m: m_it.skip(1).next().unwrap().parse::<u64>().unwrap(),
            a: a_it.skip(1).next().unwrap().parse::<u64>().unwrap(),
            s: s_it.skip(1).next().unwrap().parse::<u64>().unwrap(),
        }
    }
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut wfs: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut gen_wf = true;
    input.lines().for_each(|line| {
        if line.len() == 0 {
            gen_wf = false;
            return;
        }

        if gen_wf {
            let (name, wf) = Workflow::from(line);
            wfs.insert(name, wf);
        } else {
            parts.push(Part::from(line));
        }
    });

    (wfs, parts)
}

pub fn task1(input: &str) -> u64 {
    let (wfs, parts) = parse(input);

    parts
        .iter()
        .map(|part| {
            let mut curr_wf = "in";

            while curr_wf != "A" && curr_wf != "R" {
                let wf = &wfs.get(curr_wf).unwrap();
                for i in 0..wf.rules.len() {
                    let rule = &wf.rules[i];
                    if !rule.condition {
                        curr_wf = &rule.next;
                        break;
                    } else {
                        if rule.var == "x" && rule.lt == false {
                            if part.x > rule.val {
                                curr_wf = &rule.next;
                                break;
                            }
                        }
                        if rule.var == "x" && rule.lt == true {
                            if part.x < rule.val {
                                curr_wf = &rule.next;
                                break;
                            }
                        }
                        if rule.var == "m" && rule.lt == false {
                            if part.m > rule.val {
                                curr_wf = &rule.next;
                                break;
                            }
                        }
                        if rule.var == "m" && rule.lt == true {
                            if part.m < rule.val {
                                curr_wf = &rule.next;
                                break;
                            }
                        }
                        if rule.var == "a" && rule.lt == false {
                            if part.a > rule.val {
                                curr_wf = &rule.next;
                                break;
                            }
                        }
                        if rule.var == "a" && rule.lt == true {
                            if part.a < rule.val {
                                curr_wf = &rule.next;
                                break;
                            }
                        }
                        if rule.var == "s" && rule.lt == false {
                            if part.s > rule.val {
                                curr_wf = &rule.next;
                                break;
                            }
                        }
                        if rule.var == "s" && rule.lt == true {
                            if part.s < rule.val {
                                curr_wf = &rule.next;
                                break;
                            }
                        }
                    }
                }
            }

            if curr_wf == "R" {
                0
            } else {
                part.x + part.m + part.a + part.s
            }
        })
        .sum()
}

#[test]
fn part_test() {
    let part = Part::from("{x=1679,m=44,a=2067,s=496}");
    assert_eq!(part.x, 1679);
    assert_eq!(part.m, 44);
    assert_eq!(part.a, 2067);
    assert_eq!(part.s, 496);
}

#[test]
fn wf_test() {
    let (name, wf) = Workflow::from("qqz{s>2770:qs,m<1801:hdj,R}");
    assert_eq!(name, "qqz");
    assert_eq!(wf.rules.len(), 3);
    assert_eq!(wf.rules[0].condition, true);
    assert_eq!(wf.rules[0].var, "s");
    assert_eq!(wf.rules[0].lt, false);
    assert_eq!(wf.rules[0].val, 2770);
    assert_eq!(wf.rules[0].next, "qs");
    assert_eq!(wf.rules[1].condition, true);
    assert_eq!(wf.rules[1].var, "m");
    assert_eq!(wf.rules[1].lt, true);
    assert_eq!(wf.rules[1].val, 1801);
    assert_eq!(wf.rules[1].next, "hdj");
    assert_eq!(wf.rules[2].condition, false);
    assert_eq!(wf.rules[2].var, "");
    assert_eq!(wf.rules[2].lt, false);
    assert_eq!(wf.rules[2].val, 0);
    assert_eq!(wf.rules[2].next, "R");
}

#[test]
fn task1_test() {
    let data = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;
    assert_eq!(task1(data), 19114);
}
