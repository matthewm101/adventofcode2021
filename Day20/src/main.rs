use std::fs;

fn get(image: &Vec<Vec<usize>>, padding: usize, x: isize, y: isize) -> usize {
    return if y >= 0 && y < image.len() as isize && x >= 0 && x < image[0].len() as isize {
        image[y as usize][x as usize]
    } else {
        padding
    }
}

fn expand(image: &Vec<Vec<usize>>, algorithm: &Vec<usize>, padding: usize) -> (Vec<Vec<usize>>, usize) {
    let mut new_image: Vec<Vec<usize>> = Vec::new();
    for y in -1isize..(image.len() as isize + 1) {
        let mut new_row: Vec<usize> = Vec::new();
        for x in -1isize..(image[0].len() as isize + 1) {
            let mut i = get(image, padding, x-1, y-1);
            i = i * 2 + get(image, padding, x, y-1);
            i = i * 2 + get(image, padding, x+1, y-1);
            i = i * 2 + get(image, padding, x-1, y);
            i = i * 2 + get(image, padding, x, y);
            i = i * 2 + get(image, padding, x+1, y);
            i = i * 2 + get(image, padding, x-1, y+1);
            i = i * 2 + get(image, padding, x, y+1);
            i = i * 2 + get(image, padding, x+1, y+1);
            new_row.push(algorithm[i]);
        }
        new_image.push(new_row);
    }
    let new_padding = if padding == 0 {algorithm[0]} else {algorithm[0x1FF]};
    return (new_image, new_padding);
}

fn count_lit(image: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for row in image {
        for &v in row {
            count += v;
        }
    }
    return count;
}

// fn print_image(image: &Vec<Vec<usize>>) {
//     for row in image {
//         for &v in row {
//             if v == 1 {print!("#")} else {print!(".")}
//         }
//         println!()
//     }
// }

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let algorithm: Vec<usize> = lines.next().unwrap().chars().map(|c|if c=='#' {1} else {0}).collect();
    lines.next();
    let mut image: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        image.push(line.chars().map(|c|if c=='#' {1} else {0}).collect());
    }
    let mut padding = 0usize;
    for _ in 0..2 {
        let (new_image, new_padding) = expand(&image, &algorithm, padding);
        image = new_image;
        padding = new_padding;
    }
    println!("After 2 steps, {} pixels are lit", count_lit(&image));
    for _ in 0..48 {
        let (new_image, new_padding) = expand(&image, &algorithm, padding);
        image = new_image;
        padding = new_padding;
    }
    println!("After 50 steps, {} pixels are lit", count_lit(&image));
}
