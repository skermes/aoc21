use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Seven Segment Search";

#[derive(Debug)]
struct Display {
    digit_signals: Vec<HashSet<char>>,
    outputs: Vec<HashSet<char>>
}

impl FromStr for Display {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signals, outputs) = s.split_once(" | ")
            .ok_or_else(|| AocError::Misc(format!("Invalid display string \"{}\"", s)))?;

        Ok(Display {
            digit_signals: signals.split(" ").map(|s| s.chars().collect()).collect(),
            outputs: outputs.split(" ").map(|s| s.chars().collect()).collect()
        })
    }
}

const A: usize = 0b0000001;
const B: usize = 0b0000010;
const C: usize = 0b0000100;
const D: usize = 0b0001000;
const E: usize = 0b0010000;
const F: usize = 0b0100000;
const G: usize = 0b1000000;

impl Display {
    fn wiring_pattern(&self) -> HashMap<char, usize> {
        let mut all_signals = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);

        let mut f_and_c_set = self.digit_signals.iter()
            .filter(|signal| signal.len() == 2)
            .cloned()
            .next()
            .unwrap();

        let a_and_c_and_f_set = self.digit_signals.iter()
            .filter(|signal| signal.len() == 3)
            .next()
            .unwrap();

        let mut b_and_d_set: HashSet<char> = self.digit_signals.iter()
            .filter(|signal| signal.len() == 4)
            .next()
            .unwrap()
            .difference(&f_and_c_set)
            .map(|&c| c)
            .collect();

        let mut missing_from_sixes: HashSet<char> = self.digit_signals.iter()
            .filter(|signal| signal.len() == 6)
            .flat_map(|signal| all_signals.difference(signal))
            .map(|&c| c)
            .collect();

        let signal_a = a_and_c_and_f_set.difference(&f_and_c_set).next().unwrap().clone();
        let signal_d = missing_from_sixes.intersection(&b_and_d_set).next().unwrap().clone();
        let signal_c = missing_from_sixes.intersection(&f_and_c_set).next().unwrap().clone();

        missing_from_sixes.remove(&signal_d);
        missing_from_sixes.remove(&signal_c);
        let signal_e = missing_from_sixes.iter().next().unwrap();

        f_and_c_set.remove(&signal_c);
        let signal_f = f_and_c_set.iter().next().unwrap();

        b_and_d_set.remove(&signal_d);
        let signal_b = b_and_d_set.iter().next().unwrap();

        all_signals.remove(&signal_a);
        all_signals.remove(signal_b);
        all_signals.remove(&signal_c);
        all_signals.remove(&signal_d);
        all_signals.remove(signal_e);
        all_signals.remove(signal_f);
        let signal_g = all_signals.iter().next().unwrap();

        HashMap::from([
            (signal_a,  A),
            (*signal_b, B),
            (signal_c,  C),
            (signal_d,  D),
            (*signal_e, E),
            (*signal_f, F),
            (*signal_g, G)
        ])
    }
}

fn digit(wiring: &HashMap<char, usize>, wires: &HashSet<char>) -> usize {
    let digits: HashMap<usize, usize> = HashMap::from([
        (A + B + C + E + F + G,     0),
        (C + F,                     1),
        (A + C + D + E + G,         2),
        (A + C + D + F + G,         3),
        (B + C + D + F,             4),
        (A + B + D + F + G,         5),
        (A + B + D + E + F + G,     6),
        (A + C + F,                 7),
        (A + B + C + D + E + F + G, 8),
        (A + B + C + D + F + G,     9)
    ]);

    *digits.get(&wires.iter().map(|w| wiring.get(w).unwrap()).sum()).unwrap()
}

fn output_value(wiring: &HashMap<char, usize>, display: &Display) -> usize {
    digit(wiring, &display.outputs[0]) * 1000 +
    digit(wiring, &display.outputs[1]) * 100 +
    digit(wiring, &display.outputs[2]) * 10 +
    digit(wiring, &display.outputs[3])
}

fn is_1_or_4_or_7_or_8(pattern: &HashSet<char>) -> bool {
    pattern.len() == 2 || pattern.len() == 4 || pattern.len() == 3 || pattern.len() == 7
}

pub fn part_one(input: &str) -> Result<String, AocError> {
//     let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let displays = input.lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Display>, AocError>>()?;

    let count = displays
        .iter()
        .flat_map(|d| {
            d.outputs.iter()
                .map(|o| is_1_or_4_or_7_or_8(o))
        })
        .filter(|b| *b)
        .count();

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
//         let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let displays = input.lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Display>, AocError>>()?;

    let sum: usize = displays.iter()
        .map(|d| output_value(&d.wiring_pattern(), &d))
        .sum();

    Ok(sum.to_string())
}