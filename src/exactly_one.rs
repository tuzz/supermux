use crate::*;

const GROUP_SIZE: usize = 3;

// Based on: https://www.cs.cmu.edu/~wklieber/papers/2007_efficient-cnf-encoding-for-selecting-1.pdf

pub fn exactly_one(literals: &[i32]) {
    if literals.len() <= GROUP_SIZE {
        at_least_one(literals);
        at_most_one(literals);
    } else {
        let commanders = literals.chunks(GROUP_SIZE).map(|group| {
            let commander = SOLVER.new_literal();

            // 1. At most one variable in a group can be true.
            at_most_one(group);

            // 2. If the commander variable of a group is true, then at least
            //    one of the variables in the group must be true.
            SOLVER.add(-commander);
            at_least_one(group);

            // 3. If the commander variable of a group is false, then none of
            //    the variables in the group can be true.
            for literal in group {
                SOLVER.add(commander);
                SOLVER.add(-literal);
                SOLVER.add(0);
            }

            commander
        }).collect::<Vec<_>>();

        // 4. Exactly one of the commander variables is true.
        exactly_one(&commanders);
    }
}

fn at_least_one(literals: &[i32]) {
    for literal in literals {
        SOLVER.add(*literal);
    }

    SOLVER.add(0);
}

// Note: this shouldn't be used for group sizes > 3. Instead, use a modified
// version of exactly_one without step 2. as per the paper.
fn at_most_one(literals: &[i32]) {
    for pair in literals.iter().combinations(2) {
        SOLVER.add(-pair[0]);
        SOLVER.add(-pair[1]);
        SOLVER.add(0);
    }
}
