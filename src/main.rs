mod exactly_one;
mod multiplex;
mod solver;

use exactly_one::*;
use ipasir_sys::*;
use itertools::*;
use lazy_static::*;
use multiplex::*;
use solver::*;
use std::cell::RefCell;

fn main() {
    let length = 7;
    let bits = 8;

    let inputs = (0..length).map(|_| {
        (0..bits).map(|_| {
            SOLVER.new_literal()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let (selectors, outputs) = multiplex(&inputs);

    println!("{}", selectors.len());

    for i in 0..length {
        for j in 0..bits {
            if i == j {
                SOLVER.add(inputs[i][j]);
                SOLVER.add(0);
            } else {
                SOLVER.add(-inputs[i][j]);
                SOLVER.add(0);
            }
        }
    }

    // Set the selector to index 5
    SOLVER.add(selectors[0]);
    SOLVER.add(0);
    SOLVER.add(-selectors[1]);
    SOLVER.add(0);
    SOLVER.add(selectors[2]);
    SOLVER.add(0);

    let status = SOLVER.run();
    println!("status: {}", status);

    for i in 0..length {
        for j in 0..bits {
            let input = SOLVER.assignment(inputs[i][j]);
            print!("{}", if input { 1 } else { 0 });
        }
        println!();
    }

    println!("---");

    for i in 0..3 {
        let selector = SOLVER.assignment(selectors[i]);
        print!("{}", if selector { 1 } else { 0 });
    }

    println!();
    println!("---");

    for i in 0..bits {
        let output = SOLVER.assignment(outputs[i]);
        print!("{}", if output { 1 } else { 0 });
    }

    println!();
}
