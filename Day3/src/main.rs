use std::fs;

fn parse_binary_string(s: &str) -> u64 {
    let mut n = 0u64;
    for c in s.as_bytes() {
        if *c == ('1' as u8) {
            n = (n << 1) | 1;
        } else {
            n = n << 1;
        }
    }
    return n;
}
fn main() {
    let bitsize = 12;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut nums: Vec<u64> = Vec::new();
    for line in input.lines() {
        nums.push(parse_binary_string(line));
    }

    let mut trackers = vec![0i64; bitsize];
    for &num in &nums {
        for i in 0..bitsize {
            if num & (1 << i) == 0 {
                trackers[i] -= 1;
            } else {
                trackers[i] += 1;
            }
        }
    }

    let mut gamma = 0u64;
    let mut epsilon = 0u64;
    for i in 0..bitsize {
        if trackers[i] > 0 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    println!("Gamma: {}, Epsilon: {}, Power Consumption: {}", gamma, epsilon, gamma * epsilon);

    let mut oxygen = 0u64;
    let mut co2 = 0u64;
    for i in (0..bitsize).rev() {
        let mut oxygen_tracker = 0i64;
        let mut good_oxygens_found = 0u64;
        let mut last_good_oxygen_found = 0u64;
        for &num in &nums {
            let mut oxygen_good = true;
            for j in i+1..bitsize {
                if (1 << j) & num != (1 << j) & oxygen {
                    oxygen_good = false;
                }
            }
            if oxygen_good {
                if num != last_good_oxygen_found {
                    good_oxygens_found += 1;
                }
                last_good_oxygen_found = num;
                if (1 << i) & num == 0 {
                    oxygen_tracker -= 1;
                } else {
                    oxygen_tracker += 1;
                }
            }
        }
        if good_oxygens_found == 1 {
            oxygen = last_good_oxygen_found;
            break;
        }
        if oxygen_tracker >= 0 {
            oxygen |= 1 << i;
        }
    }
    for i in (0..bitsize).rev() {
        let mut co2_tracker = 0i64;
        let mut good_co2s_found = 0u64;
        let mut last_good_co2_found = 0u64;
        for &num in &nums {
            let mut co2_good = true;
            for j in i+1..bitsize {
                if (1 << j) & num != (1 << j) & co2 {
                    co2_good = false;
                }
            }
            if co2_good {
                if num != last_good_co2_found {
                    good_co2s_found += 1;
                }
                last_good_co2_found = num;
                if (1 << i) & num == 0 {
                    co2_tracker -= 1;
                } else {
                    co2_tracker += 1;
                }
            }
        }
        if good_co2s_found == 1 {
            co2 = last_good_co2_found;
            break;
        }
        if co2_tracker < 0 {
            co2 |= 1 << i;
        }
    }
    println!("Oxygen: {}, CO2: {}, Life Support: {}", oxygen, co2, oxygen * co2);
}
