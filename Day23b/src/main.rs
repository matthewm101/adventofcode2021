use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct State {
    hall: [u8;7], // Skips spaces that can't be filled
    room_a: [u8;4], // 0 is closest to the hallway
    room_b: [u8;4],
    room_c: [u8;4],
    room_d: [u8;4]
}

impl State {
    fn is_goal(&self) -> bool {
        self.room_a[0] == 1 && self.room_a[1] == 1 && self.room_a[2] == 1 && self.room_a[3] == 1 &&
        self.room_b[0] == 2 && self.room_b[1] == 2 && self.room_b[2] == 2 && self.room_b[3] == 2 &&
        self.room_c[0] == 3 && self.room_c[1] == 3 && self.room_c[2] == 3 && self.room_c[3] == 3 &&
        self.room_d[0] == 4 && self.room_d[1] == 4 && self.room_d[2] == 4 && self.room_d[3] == 4
    }
}

fn p10(t: u8) -> usize {
    match t {
        1 => 1,
        2 => 10,
        3 => 100,
        4 => 1000,
        _ => panic!()
    }
}

// #############
// #...........#
// ###A#C#C#D###
//   #B#D#A#B#
//   #########

fn check_move_out_of_room(room: &[u8;4], room_no: u8) -> Option<usize> {
    if room[0] == 0 && room[1] == 0 && room[2] == 0 && room[3] > 0 && room[3] != room_no {
        return Some(3);
    }
    if room[0] == 0 && room[1] == 0 && room[2] > 0 && (room[2] != room_no || room[3] != room_no) {
        return Some(2);
    }
    if room[0] == 0 && room[1] > 0 && (room[1] != room_no || room[2] != room_no || room[3] != room_no) {
        return Some(1);
    }
    if room[0] > 0 && (room[0] != room_no || room[1] != room_no || room[2] != room_no || room[3] != room_no) {
        return Some(0);
    }
    return None;
}

fn check_move_into_room(room: &[u8;4], room_no: u8) -> Option<usize> {
    if room[0] == 0 && room[1] == 0 && room[2] == 0 && room[3] == 0 {
        return Some(3);
    }
    if room[0] == 0 && room[1] == 0 && room[2] == 0 && room[3] == room_no {
        return Some(2);
    }
    if room[0] == 0 && room[1] == 0 && room[2] == room_no && room[3] == room_no {
        return Some(1);
    }
    if room[0] == 0 && room[1] == room_no && room[2] == room_no && room[3] == room_no {
        return Some(0);
    }
    return None;
}

fn succ(curr: &State, curr_cost: usize) -> Vec<(State,usize)> {
    let mut nexts: Vec<(State,usize)> = Vec::new();
    // Expand all states going from a room to a hallway
    if let Some(index) = check_move_out_of_room(&curr.room_a, 1) {
        let val = curr.room_a[index];
        if curr.hall[0] == 0 && curr.hall[1] == 0 {
            let mut s = curr.clone();s.hall[0] = val;s.room_a[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (3 + index)));
        }
        if curr.hall[1] == 0 {
            let mut s = curr.clone();s.hall[1] = val;s.room_a[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (2 + index)));
        }
        if curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[2] = val;s.room_a[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (2 + index)));
        }
        if curr.hall[3] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[3] = val;s.room_a[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (4 + index)));
        }
        if curr.hall[4] == 0 && curr.hall[3] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[4] = val;s.room_a[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (6 + index)));
        }
        if curr.hall[5] == 0 && curr.hall[4] == 0 && curr.hall[3] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[5] = val;s.room_a[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (8 + index)));
        }
        if curr.hall[6] == 0 && curr.hall[5] == 0 && curr.hall[4] == 0 && curr.hall[3] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[6] = val;s.room_a[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (9 + index)));
        }
    }
    if let Some(index) = check_move_out_of_room(&curr.room_b, 2) {
        let val = curr.room_b[index];
        if curr.hall[0] == 0 && curr.hall[1] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[0] = val;s.room_b[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (5 + index)));
        }
        if curr.hall[1] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[1] = val;s.room_b[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (4 + index)));
        }
        if curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[2] = val;s.room_b[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (2 + index)));
        }
        if curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[3] = val;s.room_b[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (2 + index)));
        }
        if curr.hall[4] == 0 && curr.hall[3] == 0{
            let mut s = curr.clone();s.hall[4] = val;s.room_b[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (4 + index)));
        }
        if curr.hall[5] == 0 && curr.hall[4] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[5] = val;s.room_b[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (6 + index)));
        }
        if curr.hall[6] == 0 && curr.hall[5] == 0 && curr.hall[4] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[6] = val;s.room_b[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (7 + index)));
        }
    }
    if let Some(index) = check_move_out_of_room(&curr.room_c, 3) {
        let val = curr.room_c[index];
        if curr.hall[0] == 0 && curr.hall[1] == 0 && curr.hall[2] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[0] = val;s.room_c[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (7 + index)));
        }
        if curr.hall[1] == 0 && curr.hall[2] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[1] = val;s.room_c[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (6 + index)));
        }
        if curr.hall[2] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[2] = val;s.room_c[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (4 + index)));
        }
        if curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[3] = val;s.room_c[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (2 + index)));
        }
        if curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[4] = val;s.room_c[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (2 + index)));
        }
        if curr.hall[5] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[5] = val;s.room_c[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (4 + index)));
        }
        if curr.hall[6] == 0 && curr.hall[5] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[6] = val;s.room_c[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (5 + index)));
        }
    }
    if let Some(index) = check_move_out_of_room(&curr.room_d, 4) {
        let val = curr.room_d[index];
        if curr.hall[0] == 0 && curr.hall[1] == 0 && curr.hall[2] == 0 && curr.hall[3] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[0] = val;s.room_d[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (9 + index)));
        }
        if curr.hall[1] == 0 && curr.hall[2] == 0 && curr.hall[3] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[1] = val;s.room_d[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (8 + index)));
        }
        if curr.hall[2] == 0 && curr.hall[3] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[2] = val;s.room_d[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (6 + index)));
        }
        if curr.hall[3] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[3] = val;s.room_d[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (4 + index)));
        }
        if curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[4] = val;s.room_d[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (2 + index)));
        }
        if curr.hall[5] == 0 {
            let mut s = curr.clone();s.hall[5] = val;s.room_d[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (2 + index)));
        }
        if curr.hall[6] == 0 && curr.hall[5] == 0 {
            let mut s = curr.clone();s.hall[6] = val;s.room_d[index] = 0;
            nexts.push((s, curr_cost + p10(val) * (3 + index)));
        }
    }

    // Expand all states going from the hallway to a room
    if let Some(index) = check_move_into_room(&curr.room_a, 1) {
        if curr.hall[0] == 1 && curr.hall[1] == 0 {
            let mut s = curr.clone();s.hall[0] = 0;s.room_a[index] = curr.hall[0];
            nexts.push((s, curr_cost + p10(curr.hall[0]) * (index + 3)));
        }
        if curr.hall[1] == 1 {
            let mut s = curr.clone();s.hall[1] = 0;s.room_a[index] = curr.hall[1];
            nexts.push((s, curr_cost + p10(curr.hall[1]) * (index + 2)));
        }
        if curr.hall[2] == 1 {
            let mut s = curr.clone();s.hall[2] = 0;s.room_a[index] = curr.hall[2];
            nexts.push((s, curr_cost + p10(curr.hall[2]) * (index + 2)));
        }
        if curr.hall[3] == 1 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[3] = 0;s.room_a[index] = curr.hall[3];
            nexts.push((s, curr_cost + p10(curr.hall[3]) * (index + 4)));
        }
        if curr.hall[4] == 1 && curr.hall[3] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[4] = 0;s.room_a[index] = curr.hall[4];
            nexts.push((s, curr_cost + p10(curr.hall[4]) * (index + 6)));
        }
        if curr.hall[5] == 1 && curr.hall[4] == 0 && curr.hall[3] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[5] = 0;s.room_a[index] = curr.hall[5];
            nexts.push((s, curr_cost + p10(curr.hall[5]) * (index + 8)));
        }
        if curr.hall[6] == 1 && curr.hall[5] == 0 && curr.hall[4] == 0 && curr.hall[3] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[6] = 0;s.room_a[index] = curr.hall[6];
            nexts.push((s, curr_cost + p10(curr.hall[6]) * (index + 9)));
        }
    }
    if let Some(index) = check_move_into_room(&curr.room_b, 2) {
        if curr.hall[0] == 2 && curr.hall[1] == 0 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[0] = 0;s.room_b[index] = curr.hall[0];
            nexts.push((s, curr_cost + p10(curr.hall[0]) * (index + 5)));
        }
        if curr.hall[1] == 2 && curr.hall[2] == 0 {
            let mut s = curr.clone();s.hall[1] = 0;s.room_b[index] = curr.hall[1];
            nexts.push((s, curr_cost + p10(curr.hall[1]) * (index + 4)));
        }
        if curr.hall[2] == 2 {
            let mut s = curr.clone();s.hall[2] = 0;s.room_b[index] = curr.hall[2];
            nexts.push((s, curr_cost + p10(curr.hall[2]) * (index + 2)));
        }
        if curr.hall[3] == 2 {
            let mut s = curr.clone();s.hall[3] = 0;s.room_b[index] = curr.hall[3];
            nexts.push((s, curr_cost + p10(curr.hall[3]) * (index + 2)));
        }
        if curr.hall[4] == 2 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[4] = 0;s.room_b[index] = curr.hall[4];
            nexts.push((s, curr_cost + p10(curr.hall[4]) * (index + 4)));
        }
        if curr.hall[5] == 2 && curr.hall[4] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[5] = 0;s.room_b[index] = curr.hall[5];
            nexts.push((s, curr_cost + p10(curr.hall[5]) * (index + 6)));
        }
        if curr.hall[6] == 2 && curr.hall[5] == 0 && curr.hall[4] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[6] = 0;s.room_b[index] = curr.hall[6];
            nexts.push((s, curr_cost + p10(curr.hall[6]) * (index + 7)));
        }
    }
    if let Some(index) = check_move_into_room(&curr.room_c, 3) {
        if curr.hall[0] == 3 && curr.hall[1] == 0 && curr.hall[2] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[0] = 0;s.room_c[index] = curr.hall[0];
            nexts.push((s, curr_cost + p10(curr.hall[0]) * (index + 7)));
        }
        if curr.hall[1] == 3 && curr.hall[2] == 0 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[1] = 0;s.room_c[index] = curr.hall[1];
            nexts.push((s, curr_cost + p10(curr.hall[1]) * (index + 6)));
        }
        if curr.hall[2] == 3 && curr.hall[3] == 0 {
            let mut s = curr.clone();s.hall[2] = 0;s.room_c[index] = curr.hall[2];
            nexts.push((s, curr_cost + p10(curr.hall[2]) * (index + 4)));
        }
        if curr.hall[3] == 3 {
            let mut s = curr.clone();s.hall[3] = 0;s.room_c[index] = curr.hall[3];
            nexts.push((s, curr_cost + p10(curr.hall[3]) * (index + 2)));
        }
        if curr.hall[4] == 3 {
            let mut s = curr.clone();s.hall[4] = 0;s.room_c[index] = curr.hall[4];
            nexts.push((s, curr_cost + p10(curr.hall[4]) * (index + 2)));
        }
        if curr.hall[5] == 3 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[5] = 0;s.room_c[index] = curr.hall[5];
            nexts.push((s, curr_cost + p10(curr.hall[5]) * (index + 4)));
        }
        if curr.hall[6] == 3 && curr.hall[5] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[6] = 0;s.room_c[index] = curr.hall[6];
            nexts.push((s, curr_cost + p10(curr.hall[6]) * (index + 5)));
        }
    }
    if let Some(index) = check_move_into_room(&curr.room_d, 4) {
        if curr.hall[0] == 4 && curr.hall[1] == 0 && curr.hall[2] == 0 && curr.hall[3] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[0] = 0;s.room_d[index] = curr.hall[0];
            nexts.push((s, curr_cost + p10(curr.hall[0]) * (index + 9)));
        }
        if curr.hall[1] == 4 && curr.hall[2] == 0 && curr.hall[3] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[1] = 0;s.room_d[index] = curr.hall[1];
            nexts.push((s, curr_cost + p10(curr.hall[1]) * (index + 8)));
        }
        if curr.hall[2] == 4 && curr.hall[3] == 0 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[2] = 0;s.room_d[index] = curr.hall[2];
            nexts.push((s, curr_cost + p10(curr.hall[2]) * (index + 6)));
        }
        if curr.hall[3] == 4 && curr.hall[4] == 0 {
            let mut s = curr.clone();s.hall[3] = 0;s.room_d[index] = curr.hall[3];
            nexts.push((s, curr_cost + p10(curr.hall[3]) * (index + 4)));
        }
        if curr.hall[4] == 4 {
            let mut s = curr.clone();s.hall[4] = 0;s.room_d[index] = curr.hall[4];
            nexts.push((s, curr_cost + p10(curr.hall[4]) * (index + 2)));
        }
        if curr.hall[5] == 4 {
            let mut s = curr.clone();s.hall[5] = 0;s.room_d[index] = curr.hall[5];
            nexts.push((s, curr_cost + p10(curr.hall[5]) * (index + 2)));
        }
        if curr.hall[6] == 4 && curr.hall[5] == 0 {
            let mut s = curr.clone();s.hall[6] = 0;s.room_d[index] = curr.hall[6];
            nexts.push((s, curr_cost + p10(curr.hall[6]) * (index + 3)));
        }
    }

    return nexts;
}

fn main() {
    let input: Vec<u8> = fs::read_to_string("input.txt").unwrap().chars().filter(|&c|c=='A'||c=='B'||c=='C'||c=='D').map(|c|(c as u8) - 64).collect();
    let start_state = State {
        hall: [0;7],
        room_a: [input[0], 4, 4, input[4]],
        room_b: [input[1], 3, 2, input[5]],
        room_c: [input[2], 2, 1, input[6]],
        room_d: [input[3], 1, 3, input[7]]
    };

    let mut exposed_states: HashMap<State,usize> = HashMap::new(); // States that have been pushed
    let mut rmap: HashMap<usize,State> = HashMap::new();
    let mut index = 0usize;
    let mut explored_states: HashMap<usize,usize> = HashMap::new(); // States that have been popped, maps index to cost
    let mut queue: BinaryHeap<Reverse<(usize,usize)>> = BinaryHeap::new();
    exposed_states.insert(start_state, index);
    rmap.insert(index, start_state);
    index += 1;
    queue.push(Reverse((0,0)));
    let mut lowest_cost: Option<usize> = None;
    'outerloop: while !queue.is_empty() {
        if let Some(Reverse((cost,i))) = queue.pop() {
            let state = rmap.get(&i).unwrap();
            if state.is_goal() {
                lowest_cost = Some(cost);
                break 'outerloop;
            }
            explored_states.insert(i, cost);
            'succloop: for s in succ(state,cost) {
                let succ_index = if let Some(j) = exposed_states.get(&s.0) {
                    if explored_states.contains_key(j) {
                        continue 'succloop;
                    }
                    *j
                } else {
                    exposed_states.insert(s.0.clone(), index);
                    rmap.insert(index, s.0);
                    index += 1;
                    index - 1
                };
                queue.push(Reverse((s.1, succ_index)));
            }
        }
    }
    println!("Lowest cost: {}", lowest_cost.unwrap());
}
