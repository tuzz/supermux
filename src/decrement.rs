use crate::*;

// Decrements the input N times and returns a vec of N numbers that have been decremented by 0..N.
// The second return value is a bit that indicates whether the decrement underflowed.
pub fn decrement_n(input: &[i32], n: usize) -> (Vec<Vec<i32>>, i32) {
    let mut previous = input.to_vec();
    let mut underflowed = SOLVER.false_literal();

    let decremented = (0..n).map(|i| {
        if i == 0 { return previous.clone(); }

        let outputs = ripple_decrement(&previous);

        previous = outputs.0.clone();
        underflowed = or(underflowed, outputs.1);

        outputs.0
    }).collect::<Vec<_>>();

    (decremented, underflowed)
}

// Based on: http://letslearncomputing.blogspot.com/2013/03/digital-logic-4-bit-binary-decrementer.html
pub fn ripple_decrement(input: &[i32]) -> (Vec<i32>, i32) {
    let mut carry = SOLVER.false_literal();

    let mut decremented = input.iter().rev().map(|bit| { // LSB is on the right
        let outputs = decrement(*bit, carry);

        carry = outputs.1;
        outputs.0
    }).collect::<Vec<_>>();

    decremented.reverse();

    (decremented, -carry)
}

pub fn decrement(input: i32, carry_in: i32) -> (i32, i32) {
    let xor1_out = xor(SOLVER.true_literal(), input);
    let xor2_out = xor(xor1_out, carry_in);

    let and1_out = and(carry_in, xor1_out);
    let and2_out = and(input, SOLVER.true_literal());

    let or_out = or(and1_out, and2_out);

    (xor2_out, or_out)
}
