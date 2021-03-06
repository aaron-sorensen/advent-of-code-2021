type Input = (Vec<u16>, usize);

pub fn part_1() -> usize {
    get_power_consumption(&get_input())
}

pub fn part_2() -> usize {
    get_life_support_rating(&get_input())
}

fn get_input() -> (Vec<u16>, usize) {
    include_str!("input/day-3.txt").lines().enumerate().fold(
        (Vec::new(), 0),
        |mut acc, (_i, line)| {
            acc.1 = line.len();
            acc.0.push(u16::from_str_radix(&line, 2).unwrap());
            acc
        },
    )
}

fn get_power_consumption(input: &Input) -> usize {
    let gamma_rate = most_common_bit(&input, 1);
    let epsilon_rate = inverse_bits(&gamma_rate, input.1);
    gamma_rate as usize * epsilon_rate as usize
}

fn get_life_support_rating(input: &Input) -> usize {
    let oxygen_generator_rating = get_oxygen_generator_rating(input);
    let co2_scrubber_rating = get_co2_scrubber_rating(input);
    oxygen_generator_rating as usize * co2_scrubber_rating as usize
}

fn get_co2_scrubber_rating(input: &Input) -> u16 {
    let mut exponent = (2 as u16).pow(input.1 as u32 - 1) as u16;
    let mut most_common_bits = most_common_bit(input, 1);
    let mut result = input.0.clone();

    while result.len() > 1 {
        // starting left to right filter out numbers greater than/eql to that bit
        if most_common_bits & exponent == exponent {
            // Filter out all 1's
            result = result
                .iter()
                .filter(|x| *x & exponent == 0)
                .cloned()
                .collect();
        } else {
            // Filter out all 0's
            result = result
                .iter()
                .filter(|x| *x & exponent == exponent)
                .cloned()
                .collect();
        }
        exponent = exponent / 2;
        most_common_bits = most_common_bit(&(result.clone(), input.1), 1);
    }

    result[0]
}

fn get_oxygen_generator_rating(input: &Input) -> u16 {
    let mut exponent = (2 as u16).pow(input.1 as u32 - 1) as u16;
    let mut most_common_bits = most_common_bit(input, 1);
    let mut result = input.0.clone();

    while result.len() > 1 {
        // starting left to right filter out numbers greater than/eql to that bit
        if most_common_bits & exponent == exponent {
            // Filter out all 0's
            result = result
                .iter()
                .filter(|x| *x & exponent == exponent)
                .cloned()
                .collect();
        } else {
            // Filter out all 1's
            result = result
                .iter()
                .filter(|x| *x & exponent == 0)
                .cloned()
                .collect();
        }
        exponent = exponent / 2;
        most_common_bits = most_common_bit(&(result.clone(), input.1), 1);
    }

    result[0]
}

fn most_common_bit(input: &Input, default: u16) -> u16 {
    let mut vec: Vec<u16> = Vec::new();
    for _i in 0..input.1 {
        vec.push(0);
    }
    let mut exponent: u16 = 1;

    let mut bit_counts = input.0.iter().fold(vec, |mut acc, val| {
        for i in 0..input.1 {
            acc[i] += (exponent & *val).count_ones() as u16;
            exponent = exponent * 2;
        }
        exponent = 1;
        acc
    });
    bit_counts.reverse();

    exponent = (2 as u16).pow(input.1 as u32 - 1) as u16;

    bit_counts.iter().fold(0, |mut acc, val| {
        if val > &(input.0.len() as u16 - val) {
            acc += exponent;
        } else if val == &(input.0.len() as u16 - val) {
            acc += default * exponent;
        }
        exponent = exponent / 2;
        acc
    })
}

fn inverse_bits(input: &u16, length: usize) -> u16 {
    let max_exponent = (2 as u16).pow(length as u32 - 1) as u16;
    let mut exponent = 0b1000000000000000;

    let mut flipped = !input;

    // Remove leading 1's
    while exponent > max_exponent {
        flipped -= exponent;
        exponent = exponent / 2;
    }

    flipped
}

#[test]
fn check() {
    assert_eq!(2954600, part_1());
    assert_eq!(1662846, part_2());
}
