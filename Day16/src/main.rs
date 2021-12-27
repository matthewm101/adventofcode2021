use std::fs;

struct Stream{
    bits: Vec<u64>,
    cursor: usize
}


enum Packet {
    Literal{version: u64, value: u64},
    Operator{version: u64, type_id: u64, packets: Vec<Packet>}
}

impl Stream {
    fn read_version(&mut self) -> u64 {
        let v = self.bits[self.cursor] * 4 + self.bits[self.cursor+1] * 2 + self.bits[self.cursor+2];
        self.cursor += 3;
        return v;
    }

    fn read_type(&mut self) -> u64 {
        let v = self.bits[self.cursor] * 4 + self.bits[self.cursor+1] * 2 + self.bits[self.cursor+2];
        self.cursor += 3;
        return v;
    }

    fn read_nibble(&mut self) -> u64 {
        let v = self.bits[self.cursor] * 8 + self.bits[self.cursor+1] * 4 + self.bits[self.cursor+2] * 2 + self.bits[self.cursor+3];
        self.cursor += 4;
        return v;
    }

    fn read_bool(&mut self) -> bool {
        let v = self.bits[self.cursor] != 0;
        self.cursor += 1;
        return v;
    }

    fn read_literal(&mut self) -> u64 {
        let mut keep_going = true;
        let mut v = 0u64;
        while keep_going {
            keep_going = self.read_bool();
            v = v * 16 + self.read_nibble();
        }
        return v;
    }

    fn read_15bit(&mut self) -> u64 {
        let a = self.read_type();
        let b = self.read_nibble();
        let c = self.read_nibble();
        let d = self.read_nibble();
        return a * 4096 + b * 256 + c * 16 + d;
    }

    fn read_11bit(&mut self) -> u64 {
        let a = self.read_type();
        let b = self.read_nibble();
        let c = self.read_nibble();
        return a * 256 + b * 16 + c;
    }

    fn read_several_packets(&mut self) -> Vec<Packet> {
        let mut packets = Vec::new();
        if self.read_bool() {
            let num_packets = self.read_11bit();
            for _ in 0..num_packets {
                packets.push(self.read_packet());
            }
        } else {
            let bits_to_read = self.read_15bit() as usize;
            let goal_cursor = self.cursor + bits_to_read;
            while self.cursor < goal_cursor {
                packets.push(self.read_packet());
            }
        }
        assert!(!packets.is_empty());
        return packets;
    }

    fn read_packet(&mut self) -> Packet {
        let version = self.read_version();
        let type_id = self.read_type();
        match type_id {
            4 => Packet::Literal {version, value: self.read_literal()},
            _ => Packet::Operator {version, type_id, packets: self.read_several_packets()}
        }
    }
}

fn recursively_add_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal {version, value: _} => *version,
        Packet::Operator {version, type_id: _, packets} => *version + packets.iter().map(|p|recursively_add_versions(p)).sum::<u64>()
    }
}

fn recursively_evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal {version: _, value} => *value,
        Packet::Operator {version: _, type_id, packets} => match type_id {
            0 => packets.iter().map(|p|recursively_evaluate(p)).reduce(|x,y|x+y).unwrap(),
            1 => packets.iter().map(|p|recursively_evaluate(p)).reduce(|x,y|x*y).unwrap(),
            2 => packets.iter().map(|p|recursively_evaluate(p)).reduce(|x,y|x.min(y)).unwrap(),
            3 => packets.iter().map(|p|recursively_evaluate(p)).reduce(|x,y|x.max(y)).unwrap(),
            5 => (recursively_evaluate(&packets[0]) > recursively_evaluate(&packets[1])) as u64,
            6 => (recursively_evaluate(&packets[0]) < recursively_evaluate(&packets[1])) as u64,
            7 => (recursively_evaluate(&packets[0]) == recursively_evaluate(&packets[1])) as u64,
            _ => panic!("bad type")
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut stream = Stream{bits: input.chars().map(|c| match c {
        '0' => vec![0,0,0,0],
        '1' => vec![0,0,0,1],
        '2' => vec![0,0,1,0],
        '3' => vec![0,0,1,1],
        '4' => vec![0,1,0,0],
        '5' => vec![0,1,0,1],
        '6' => vec![0,1,1,0],
        '7' => vec![0,1,1,1],
        '8' => vec![1,0,0,0],
        '9' => vec![1,0,0,1],
        'A' => vec![1,0,1,0],
        'B' => vec![1,0,1,1],
        'C' => vec![1,1,0,0],
        'D' => vec![1,1,0,1],
        'E' => vec![1,1,1,0],
        'F' => vec![1,1,1,1],
        _ => vec![]
    }).reduce(|x,y| x.into_iter().chain(y.into_iter()).collect()).unwrap(), cursor: 0};

    let packet = stream.read_packet();

    println!("Sum of all versions: {}", recursively_add_versions(&packet));
    println!("Packet evaluation: {}", recursively_evaluate(&packet));
}
