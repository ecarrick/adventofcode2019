use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;


// A struct with two fields
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Line 
{
    p: Point,
    q: Point,
}
impl Line {
    pub fn is_hor(&self) -> bool {
        if self.p.y == self.q.y {
            return true;
        }
        return false;
    }
    pub fn is_vert(&self) -> bool {
        if self.p.x == self.q.x {
            return true;
        }
        return false;
    }

}

fn find_intersect(l1: Line, l2: Line) -> Point {
    if l1.is_hor() && l2.is_vert() {
        // println!("vl1: {:?} vl2: {:?}", l1.is_hor(), l2.is_vert());
        if l2.p.x < cmp::max(l1.p.x,l1.q.x)
        && l2.p.x > cmp::min(l1.p.x,l1.q.x)
        && l1.p.y < cmp::max(l2.p.y,l2.q.y)
        && l1.p.y > cmp::min(l2.p.y,l2.q.y)
        {
            return Point{x:l2.p.x,y:l1.p.y};
        }
    }
    else if l2.is_hor() && l1.is_vert() {
        if l1.p.x < cmp::max(l2.p.x,l2.q.x)
        && l1.p.x > cmp::min(l2.p.x,l2.q.x)
        && l2.p.y < cmp::max(l1.p.y,l1.q.y)
        && l2.p.y > cmp::min(l1.p.y,l1.q.y)
        {
            return Point{x:l1.p.x,y:l2.p.y};
        }
    }

    return Point{x:0,y:0};

}


fn main() {
    let f = BufReader::new(File::open("src/input.txt").unwrap());

    // extract both paths into vectors of actions
    let paths: Vec<Vec<String>> = f.lines()
        .map(|line| line.unwrap().trim().split(",")
                    .map(|action| action.into()).collect()
        )
        .collect();

    // let mut pathpoints1: Vec<Vec<Point>> = Vec::new();
    let mut pathpoints = vec![vec![Point{x:0,y:0}], vec![Point{x:0,y:0}]];

    // setup path XY Points
    // println!("{:?}", paths[0]);
    for (ind,path) in paths.iter().enumerate() {
        for (i, action) in path.iter().enumerate() {
            let direction: String = action[0..1].to_string();
            let distance = &action[1..action.len()].parse::<i32>().unwrap();
            // println!("dir: {} dis: {}", direction, distance);
            let mut current_point = pathpoints[ind][i];
            match direction.as_ref() {
                "U" => current_point.y += distance,
                "D" => current_point.y -= distance,
                "L" => current_point.x -= distance,
                "R" => current_point.x += distance,
                _ => panic!("unkown direction!"),
            }
            pathpoints[ind].push(current_point)
        }
    }
    println!("{:?}", pathpoints);

    let mut intersects = Vec::new();


    for i in 1..pathpoints[1].len() {
        // let p1 = pathpoints[1][i-1];
        // let q1 = pathpoints[1][i];
        let line1 = Line{p:pathpoints[1][i-1], q:pathpoints[1][i]};
        // println!("p1: {:?} q1: {:?}", p1, q1);

        for j in 1..pathpoints[0].len() {
            // let p2 = pathpoints[0][j-1];
            // let q2 = pathpoints[0][j];
            let line2 = Line{p:pathpoints[0][j-1], q:pathpoints[0][j]};
            // println!("p2: {:?} q2: {:?}", p2, q2);
            let intersection = find_intersect(line1,line2);
            if intersection.x == 0 && intersection.y == 0 {
                continue;
            }
            else {
                intersects.push(intersection);
            }


        }
    }

    println!("{:?}", intersects);
    if intersects.len() > 0{
        let mut min_p = intersects[0];
        for p in intersects {
            if p.x.abs()+p.y.abs() < min_p.x.abs()+min_p.y.abs() {
                min_p = p;
            }
        }
        println!("{}", min_p.x.abs()+min_p.y.abs())
    }
    

}