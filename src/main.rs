mod exactly_one;
mod multiplex;
mod primitives;
mod solver;

use exactly_one::*;
use itertools::*;
use lazy_static::*;
use multiplex::*;
use primitives::*;
use solver::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IOWrite;
use std::iter::repeat;
use std::process::Command;
use std::str::from_utf8;

fn main() {
    let n = 4;
    let length = 33;

    let input = input_string(n, length);
    let multiplexors = shifted_multiplexors(n, length, &input);

    let address = &multiplexors[0].0;
    let mut perm_addresses = vec![];

    starts_with_ascending_numbers(&input);
    has_one_number_per_index(&input);
    set_to_same_address(&multiplexors);

    for permutation in (0..n).permutations(n) {
        let perm_address = create_address(address.len());
        let is_current = addresses_equal(&address, &perm_address);

        for (digit, (_, output)) in permutation.iter().zip(&multiplexors) {
            implies(is_current, output[*digit]);
        }

        perm_addresses.push(perm_address);
    }

    print_stats(n, length);

    for i in 1..=(SOLVER.literals() as i32) {
        let is_input = input.iter().any(|v| v.contains(&i));
        let is_address = address.contains(&i);
        let is_perm_address = perm_addresses.iter().any(|v| v.contains(&i));

        if is_input || is_perm_address { // I pick the input and perm addresses
            SOLVER.existential(i);
        } else if is_address {           // You pick any address of the mux
            SOLVER.universal(i);
        } else {                         // I try to satisfy the formula
            SOLVER.inner(i);
        }
    }

    let success = SOLVER.run();
    println!("Success: {}", success);

    if success {
        print_string(&input);
        //print_addresses(&perm_addresses, n);
    }
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
    let all_false = (0..n).map(|_| {
        let literal = SOLVER.new_literal();

        SOLVER.add(-literal);
        SOLVER.add(0);

        literal
    }).collect::<Vec<_>>();

    (0..n).map(|i| {
        multiplex(&input.iter().skip(i).chain(repeat(&all_false)).take(length).collect::<Vec<_>>())
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

fn set_to_same_address(multiplexors: &[(Vec<i32>, Vec<i32>)]) {
    let addresses = multiplexors.iter().map(|(a, _)| a).collect::<Vec<_>>();
    let first = &addresses[0];

    let equals = addresses.iter().skip(1).map(|address| {
        addresses_equal(first, address)
    }).collect::<Vec<_>>();

    SOLVER.add(all(&equals));
    SOLVER.add(0);
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

fn print_addresses(addresses: &[Vec<i32>], n: usize) {
    println!("-----");
    for (permutation, address) in (0..n).permutations(n).zip(addresses) {
        let perm = permutation.iter().map(|p| (p + 1).to_string()).collect::<Vec<_>>();
        let addr = address.iter().map(|l| SOLVER.assignment(*l)).collect::<Vec<_>>();

        println!("{} is at address {:?}", perm.join(""), addr);
    }
}
