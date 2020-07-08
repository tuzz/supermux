mod exactly_one;
mod solver;

use exactly_one::*;
use ipasir_sys::*;
use itertools::*;
use lazy_static::*;
use solver::*;
use std::cell::RefCell;

fn main() {
    let n = 10000;
    let literals = (1..=n).map(|_| SOLVER.new_literal()).collect::<Vec<_>>();

    exactly_one(&literals);

    for i in 1..=n {
        SOLVER.add(-i);
        SOLVER.add(0);
    }

    let status = SOLVER.run();

    println!("{}, {}, {}", status, SOLVER.literals(), SOLVER.clauses());
}
