mod exactly_one;
mod multiplex;
mod primitives;
mod solver;

use exactly_one::*;
use ipasir_sys::*;
use itertools::*;
use lazy_static::*;
use multiplex::*;
use primitives::*;
use solver::*;
use std::cell::RefCell;

fn main() {
    let n = 3;
    let length = 8;

    let input = input_string(n, length);
    let multiplexors = shifted_multiplexors(n, length, &input);
    let address = &multiplexors[0].0;

    starts_with_ascending_numbers(&input);
    has_one_number_per_index(&input);

    let mut foo = vec![];
    let mut bar = vec![];
    for permutation in (0..n).permutations(n) {
        let perm_address = create_address(address.len());
        let is_current = addresses_equal(&address, &perm_address);
        foo.push(is_current);
        bar.push(perm_address);

        for (digit, (_, output)) in permutation.iter().zip(&multiplexors) {
            implies(is_current, output[*digit]);
        }
    }

    print_stats(n, length);

    SOLVER.add(address[0]);
    SOLVER.add(0);
    SOLVER.add(address[1]);
    SOLVER.add(0);
    SOLVER.add(-address[2]);
    SOLVER.add(0);

    let status = SOLVER.run();
    println!("Status: {}", status);

    print_string(&input);

    for b in bar {
        let x = b.iter().map(|x| SOLVER.assignment(*x)).collect::<Vec<_>>();
        println!("{:?}", x);
    }
    println!("-----");

    println!("{}", SOLVER.assignment(address[0]));
    println!("{}", SOLVER.assignment(address[1]));
    println!("{}", SOLVER.assignment(address[2]));
    println!("-----");

    println!("{}", SOLVER.assignment(foo[0]));
    println!("{}", SOLVER.assignment(foo[1]));
    println!("{}", SOLVER.assignment(foo[2]));
    println!("{}", SOLVER.assignment(foo[3]));
    println!("{}", SOLVER.assignment(foo[4]));
    println!("{}", SOLVER.assignment(foo[5]));
}

fn input_string(n: usize, length: usize) -> Vec<Vec<i32>> {
    (0..length).map(|_| {
        (0..n).map(|_| {
            SOLVER.new_literal()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

// TODO: consider using an adder on the address instead of muxing several times.
fn shifted_multiplexors(n: usize, length: usize, input: &[Vec<i32>]) -> Vec<(Vec<i32>, Vec<i32>)> {
    (0..n).map(|i| {
        multiplex(&input.iter().cycle().skip(i).take(length).collect::<Vec<_>>())
    }).collect::<Vec<_>>()
}

fn starts_with_ascending_numbers(input: &[Vec<i32>]) {
    for i in 0..input[0].len() {
        SOLVER.add(input[i][i]);
        SOLVER.add(0);
    }
}

fn has_one_number_per_index(input: &[Vec<i32>]) {
    for one_hot in input {
        exactly_one(one_hot);
    }
}

fn create_address(length: usize) -> Vec<i32> {
    (0..length).map(|_| SOLVER.new_literal()).collect()
}

fn addresses_equal(address1: &[i32], address2: &[i32]) -> i32 {
    let equals = address1.iter().zip(address2)
        .map(|(a, b)| xnor(*a, *b))
        .collect::<Vec<_>>();

    all(&equals)
}

fn all(literals: &[i32]) -> i32 {
    match literals.len() {
        1 => literals[0],
        2 => and(literals[0], literals[1]),
        n => {
            let left = all(&literals[0..n / 2]);
            let right = all(&literals[n / 2..]);

            and(left, right)
        }
    }
}

fn implies(premise: i32, consequence: i32) {
    SOLVER.add(-premise);
    SOLVER.add(consequence);
    SOLVER.add(0);
}

fn print_stats(n: usize, length: usize) {
    println!("N: {}", n);
    println!("Length: {}", length);
    println!("Literals: {}", SOLVER.literals());
    println!("Clauses: {}", SOLVER.clauses());
    println!("-----");
}

fn print_string(input: &[Vec<i32>]) {
    for one_hot in input {
        for (i, literal) in one_hot.iter().enumerate() {
            if SOLVER.assignment(*literal) {
                print!("{}", i + 1);
            }
        }
    }
    println!();
}
