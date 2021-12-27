use std::fs;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Dir {
    FORWARD, DOWN, UP
}
#[derive(Copy, Clone)]
struct Movement {
    dir: Dir,
    dist: i64
}
impl Movement {
    fn to_vector(&self) -> (i64,i64) {
        return match self.dir {
            Dir::FORWARD => (self.dist,0),
            Dir::DOWN => (0,-self.dist),
            Dir::UP => (0,self.dist)
        }
    }
}
impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Dir::FORWARD),
            "down" => Ok(Dir::DOWN),
            "up" => Ok(Dir::UP),
            _ => Err(())
        }
    }
}
impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(" ");
        Ok(Movement { dir: splits.next().unwrap().parse().unwrap(), dist: splits.next().unwrap().parse().unwrap() })
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut movements: Vec<Movement> = Vec::new();
    for line in input.lines() {
        movements.push(line.parse().unwrap());
    }

    let mut basic_position = (0i64, 0i64);
    for movement in &movements {
        let offset = movement.to_vector();
        basic_position.0 += offset.0;
        basic_position.1 += offset.1;
    }
    println!("Final position by first method: ({},{})", basic_position.0, basic_position.1);
    println!("Multiplication of components: {}", basic_position.0 * basic_position.1 * -1);

    let mut aim = 0i64;
    let mut advanced_position = (0i64, 0i64);
    for movement in &movements {
        match movement.dir {
            Dir::FORWARD => {
                advanced_position.0 += movement.dist;
                advanced_position.1 += aim * movement.dist;
            },
            Dir::DOWN => {
                aim -= movement.dist;
            },
            Dir::UP => {
                aim += movement.dist;
            }
        };
    }
    println!("Final position by second method: ({},{})", advanced_position.0, advanced_position.1);
    println!("Multiplication of components: {}", advanced_position.0 * advanced_position.1 * -1);
}
