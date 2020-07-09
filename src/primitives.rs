use crate::*;

// These are some primitive logic gates reduced to SAT via the Tseitin transform.

pub fn and(a: i32, b: i32) -> i32 {
    let out = SOLVER.new_literal();

    SOLVER.add(-a);
    SOLVER.add(-b);
    SOLVER.add(out);
    SOLVER.add(0);

    SOLVER.add(a);
    SOLVER.add(-out);
    SOLVER.add(0);

    SOLVER.add(b);
    SOLVER.add(-out);
    SOLVER.add(0);

    out
}

pub fn or(a: i32, b: i32) -> i32 {
    let out = SOLVER.new_literal();

    SOLVER.add(a);
    SOLVER.add(b);
    SOLVER.add(-out);
    SOLVER.add(0);

    SOLVER.add(-a);
    SOLVER.add(out);
    SOLVER.add(0);

    SOLVER.add(-b);
    SOLVER.add(out);
    SOLVER.add(0);

    out
}

pub fn xnor(a: i32, b: i32) -> i32 {
    let out = SOLVER.new_literal();

    SOLVER.add(-a);
    SOLVER.add(-b);
    SOLVER.add(out);
    SOLVER.add(0);

    SOLVER.add(a);
    SOLVER.add(b);
    SOLVER.add(out);
    SOLVER.add(0);

    SOLVER.add(a);
    SOLVER.add(-b);
    SOLVER.add(-out);
    SOLVER.add(0);

    SOLVER.add(-a);
    SOLVER.add(b);
    SOLVER.add(-out);
    SOLVER.add(0);

    out
}

pub fn xor(a: i32, b: i32) -> i32 {
    let out = SOLVER.new_literal();

    SOLVER.add(-a);
    SOLVER.add(-b);
    SOLVER.add(-out);
    SOLVER.add(0);

    SOLVER.add(a);
    SOLVER.add(b);
    SOLVER.add(-out);
    SOLVER.add(0);

    SOLVER.add(a);
    SOLVER.add(-b);
    SOLVER.add(out);
    SOLVER.add(0);

    SOLVER.add(-a);
    SOLVER.add(b);
    SOLVER.add(out);
    SOLVER.add(0);

    out
}
