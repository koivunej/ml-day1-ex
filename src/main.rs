use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    // whenever you see a .unwrap() it means the value it's
    // being called on is an Option<?>; if the values was missing
    // or None, the process will exit.

    let training_path = std::env::args().nth(1).unwrap();
    let validation_path = std::env::args().nth(2).unwrap();

    let training_examples = read_file(&training_path).unwrap();
    let validation_examples = read_file(&validation_path).unwrap();

    let mut correct = 0;
    let mut wrong = 0;

    for ex in validation_examples {
        let classification = classify(&training_examples, &ex.pixels);
        if classification == ex.label {
            correct = correct + 1;
        } else {
            wrong = wrong + 1;
        }
    }

    println!(
        "{} correct, {} wrong = {:.2}%",
        correct,
        wrong,
        100.0 * (correct as f64) / ((correct + wrong) as f64));
}

struct Example {
    label: i32,
    pixels: Vec<i32>
}

// Result<T, E> contains either value T or error E
// io::Result<T> is shorthand or type alias for Result<T, IOError>
fn read_file(path: &String) -> std::io::Result<Vec<Example>> {
    let f = try!(File::open(path));

    let reader = BufReader::new(f);

    // this use of iterator might be losing any IO error
    // by the use of filter_map
    let ret = reader.lines()
        .filter_map(|result| result.ok())
        .skip(1)
        .map(|line| {
            let mut iter = line.split(',')
                .map(|num| num.parse::<i32>())
                .map(|result| result.unwrap());

            // take the first value as a label
            let label = iter.next().unwrap();

            // collect the rest into a vector
            let pixels = iter.collect::<Vec<i32>>();

            Example{label: label, pixels: pixels}
        }).collect::<Vec<Example>>();

    Ok(ret)
}

fn distance(lhs: &Vec<i32>, rhs: &Vec<i32>) -> f64 {
    assert!(lhs.len() == rhs.len());
    f64::sqrt(
        lhs.iter()
            .zip(rhs.iter())
            .map(|(left, right)| { (left - right).pow(2)})
            .fold(0, |sum, diff_squared| sum + diff_squared) as f64)
}

fn classify(examples: &Vec<Example>, unknown: &Vec<i32>) -> i32 {
    // this looks horribly as because of https://github.com/rust-lang/rfcs/issues/675
    // cannot use iter().min_by(... -> f64) as f64 is only partial_ord
    // https://crates.io/crates/ord_subset cannot help here as it only provides min/max

    let mut min: Option<(&Example, f64)> = None;

    for ex in examples.iter() {
        let d = distance(unknown, &ex.pixels);
        min = match min {
            None => Some((&ex, d)),
            Some((other_ex, other_d)) => {
                if d.min(other_d) == d {
                    Some((&ex, d))
                } else {
                    Some((&other_ex, other_d))
                }
            }
        };
    }

    let (ex, _) = min.unwrap();
    return ex.label;

    /*
    // should had been:
    examples.iter()
        .min_by(|ex| distance(unknown, &ex.pixels))
        .map(|(ex, d)| ex.label)
        .unwrap() // will panic if zero examples
    */
}


