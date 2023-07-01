use std::{io::{BufRead, BufReader}, fs::File};
use std::cmp::{min, max};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vector {
    x: i32,
    y: i32
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32
}

fn calculate_possible_trajectories(target_area: &TargetArea) -> usize {

    let min_y_vel = calculate_min_y_vel(target_area);
    let max_y_vel = calculate_max_y_vel(target_area);

    let min_x_vel = calculate_min_x_vel(target_area, max_y_vel);
    let max_x_vel = calculate_max_x_vel(target_area);

    println!("x range [{}, {}], y range [{}, {}]", min_x_vel, max_x_vel, min_y_vel, max_y_vel);

    (min_x_vel..max_x_vel+1).fold(0, |count, x_vel| {
        count + (min_y_vel..max_y_vel+1).fold(0, |row_count, y_vel| {
            row_count + simulate_trajectory(&Vector{x: x_vel, y: y_vel}, target_area) as usize
        })
    })    
}

fn calculate_min_x_vel(target_area: &TargetArea, max_y_vel: i32) -> i32 {

    let time_duration = 2 * max_y_vel + (1 - (2 * (target_area.min_y > 0) as i32));
    let mut min_x_vel = 0;

    if target_area.min_x > 0 {
        while (min_x_vel + max(min_x_vel - time_duration, 0) + 1) * min(min_x_vel, time_duration) / 2 < target_area.min_x {
            min_x_vel += 1;
        }
    } else {
        while (min_x_vel + min(min_x_vel + time_duration, 0) + 1) * min(-min_x_vel, time_duration) / 2 < target_area.max_x {
            min_x_vel -= 1;
        }       
    }

    min_x_vel
}

fn calculate_max_x_vel(target_area: &TargetArea) -> i32 {

    if target_area.min_x > 0 {
        target_area.max_x
    } else {
        target_area.min_x     
    }
}

fn calculate_min_y_vel(target_area: &TargetArea) -> i32 {

    if target_area.min_y < 0 {
        target_area.min_y
    } else {
        let mut vel = 0;

        while ((vel * vel) + vel) / 2 < target_area.min_y {
            vel += 1;
        }
        vel
    }
}

fn calculate_max_y_vel(target_area: &TargetArea) -> i32 {

    if target_area.min_y > 0 {
        target_area.min_y
    } else {
        -target_area.min_y - 1
    }
}

fn simulate_trajectory(start_velocity: &Vector, target_area: &TargetArea) -> bool {

    //println!("start velocity: {:?}", start_velocity);

    let mut prob_position = Vector{x: 0, y:0};
    let mut prob_velocity = *start_velocity;

    while prob_velocity.y >= 0 || prob_position.y >= target_area.min_y {
        prob_position += prob_velocity;
        
        prob_velocity.x = max(prob_velocity.x - 1, 0);
        prob_velocity.y -= 1; // gravity

        if prob_position.x >= target_area.min_x && prob_position.x <= target_area.max_x
            && prob_position.y >= target_area.min_y && prob_position.y <= target_area.max_y {
                //println!("{:?}", start_velocity);
                return true;
        }
    }

    return false;
}

fn main() {
    
    let line = BufReader::new(File::open("input.txt").unwrap()).lines().next().unwrap().unwrap();
    let front_trimmed_line = &line[15..];
    let ranges: Vec<&str> = front_trimmed_line.split(", y=").collect();
    let x_range: Vec<&str> = ranges[0].split("..").collect();
    let y_range: Vec<&str> = ranges[1].split("..").collect();

    println!("x_range {:?} y_range {:?}", x_range, y_range);

    let target_area = TargetArea{
        min_x: x_range[0].parse().unwrap(), max_x: x_range[1].parse().unwrap(),
        min_y: y_range[0].parse().unwrap(), max_y: y_range[1].parse().unwrap() };

    println!("possible trajectories {}", calculate_possible_trajectories(&target_area));
}
