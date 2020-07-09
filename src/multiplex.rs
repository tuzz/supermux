use crate::*;

type Selector = Vec<i32>;
type Output = Vec<i32>;

pub fn multiplex(inputs: &[Vec<i32>]) -> (Selector, Output) {
    let num_selectors = (inputs.len() as f32).log2().ceil() as u32;

    let selectors = (0..num_selectors).map(|_| SOLVER.new_literal()).collect::<Vec<_>>();
    let outputs = n_way_multiplex(inputs, &selectors, 0);

    (selectors, outputs)
}

fn n_way_multiplex(inputs: &[Vec<i32>], selectors: &[i32], level: usize) -> Output {
    match inputs.len() {
        1 => {
            inputs[0].iter().map(|a| {
                let zeroes = selectors[level..].iter().map(|s| -s);
                let bits = zeroes.clone().chain(once(*a)).collect::<Vec<_>>();

                all(&bits)
            }).collect()
        },
        2 => multiplex_bits(&inputs[0], &inputs[1], selectors[level]),
        n => {
            let half = round_up_to_power_of_2(n as f32 / 2.);

            let left = n_way_multiplex(&inputs[0..half], selectors, level + 1);
            let right = n_way_multiplex(&inputs[half..], selectors, level + 1);

            multiplex_bits(&left, &right, selectors[level])
        },
    }
}

fn multiplex_bits(a: &[i32], b: &[i32], sel: i32) -> Output {
    a.iter().zip(b).map(|(a, b)| multiplex_1(*a, *b, sel)).collect()
}

fn multiplex_1(a: i32, b: i32, sel: i32) -> i32 {
    let out1 = and(a, -sel);
    let out2 = and(b, sel);

    or(out1, out2)
}

fn round_up_to_power_of_2(n: f32) -> usize {
    let mut power = 1.;
    while power < n { power *= 2.; }
    power as usize
}
