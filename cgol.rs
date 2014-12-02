use std::os;
use std::rand;
use std::rand::Rng;
use std::io::Timer;
use std::time::Duration;

use std::clone::Clone;

#[deriving(Clone)]
struct Field {
    x: uint,
    y: uint,
    active: bool
}

fn next_round(field: &mut Vec<Vec<Field>>) {
    let old_field = field.clone();
    for row in field.iter_mut() {
        for c in row.iter_mut() {
            c.active = next_vitality(&old_field, c.x, c.y);
        }
    }
}

fn next_vitality(old_field: &Vec<Vec<Field>>, x: uint, y: uint) -> bool {
    let height = old_field.len();
    let width = old_field[0].len();
    let mut allowed_x: Vec<uint> = vec![x];
    let mut allowed_y: Vec<uint> = vec![y];
    if x > 0 {
        allowed_x.push(x-1);
    }
    if x < width-1 {
        allowed_x.push(x+1);
    }
    if y > 0 {
        allowed_y.push(y-1);
    }
    if y < height-1 {
        allowed_y.push(y+1);
    }

    let mut total: int = 0;
    for i in allowed_x.iter() {
        for j in allowed_y.iter() {
            if *i == x && *j == y {
                continue;
            }
            if old_field[*j][*i].active == true {
                total += 1;
            }
        }
    }

    match total {
        2 => old_field[y][x].active,
        3 => true,
        _ => false
    }
}

fn print_field(field: &Vec<Vec<Field>>) {
    for row in field.iter() {
        for c in row.iter() {
            print!("{}", match c.active {false => " ", true => "#"});
        }
        print!("\n");
    }
    print!("\n");
}

fn generate_first_round() -> Vec<Vec<Field>> {
    let mut field: Vec<Vec<Field>> = Vec::new();
    let args = os::args();
    let mut width: uint = 50;
    let mut height: uint = 50;
    if args.len() > 1 {
      match from_str::<uint>(args[1].as_slice()){
        Some(x) => width = x,
        None    => fail!("Argument supplied is not a positive number")
      };
    } else {
      print!("Use default value 50 as width")
    }
    if args.len() > 2 {
      match from_str::<uint>(args[2].as_slice()){
        Some(x) => height = x,
        None    => fail!("Argument supplied is not a positive number")
      };
    } else {
      print!("Use default value 50 as height")
    }
    for y in range(0u, height) {
        let mut row: Vec<Field> = Vec::new();

        for x in range(0u, width) {
        let mut v = false;
            let mut rng = rand::task_rng();
            
            
            if rng.gen::<u8>() < 32 {
                v = true; 
            } else {
                v = false;
            }

            row.push(Field{x:x, y:y, active:v});
        }
        field.push(row);
    }
    return field;
}

fn main() {
    let mut field = generate_first_round();
    let mut timer = Timer::new().unwrap();
    loop {
        timer.sleep(Duration::milliseconds(150));

        print_field(&field);
        next_round(&mut field);
    }
}
