use std::cmp;
use strum_macros::EnumIter;
// state obohatit
pub struct Enviroment {
    width: u32,
    height: u32,
    begin_x: i32,
    begin_y: i32,
    pub x: i32,
    pub y: i32,
    goal_x: i32,
    goal_y: i32,
    distance: f64,
    obstacles: Vec<(u32, u32, u32, u32)>,
}
impl Enviroment {
    pub fn new(
        width: u32,
        height: u32,
        begin_x: i32,
        begin_y: i32,
        goal_x: i32,
        goal_y: i32,
    ) -> Self {
        Enviroment {
            width,
            height,
            begin_x,
            begin_y,
            x: begin_x,
            y: begin_y,
            goal_x,
            goal_y,
            distance: (((begin_x - goal_x).pow(2) + (begin_y - goal_y).pow(2)) as f64).sqrt(),
            obstacles: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.x = self.begin_x;
        self.y = self.begin_y;
    }
    pub fn step(&mut self, action: &Action) -> (i32, bool) {
        match action {
            Action::Up => {
                self.y -= 1;
            }
            Action::Down => {
                self.y += 1;
            }
            Action::Left => {
                self.x -= 1;
            }
            Action::Right => {
                self.x += 1;
            }
        }
        if self.collision() {
            return (-10, true);
        }
        if self.x == self.goal_x && self.y == self.goal_y {
            return (10, true);
        }
        let new_distance = self.get_distance();
        let compare: bool = self.distance < new_distance;
        self.distance = new_distance;
        if compare {
            (-1, false)
        } else {
            (1, false)
        }
    }

    pub fn add_obstacle(&mut self, obstacle: (u32, u32, u32, u32)) {
        self.obstacles.push(obstacle);
    }

    pub fn obstacle_distance(&mut self) -> [i32; 4] {
        let mut x1: i32 = 0;
        let mut x2: i32 = self.width as i32 - 1;
        let mut y1: i32 = 0;
        let mut y2: i32 = self.height as i32 - 1;
        for &(x, y, w, h) in self.obstacles.iter() {
            match self.obstacle_position((x, y, w, h)) {
                Place::Above => y1 = cmp::max(y1, y as i32 + h as i32),
                Place::Under => y2 = cmp::min(y2, y as i32 - 1),
                Place::Left => x1 = cmp::max(x1, x as i32 + w as i32),
                Place::Right => x2 = cmp::min(x2, x as i32 - 1),
                Place::None => continue,
            }
        }
        [self.y - y1, x2 - self.x, y2 - self.y, self.x - x1]
    }

    pub fn obstacle_position(&self, obstacle: (u32, u32, u32, u32)) -> Place {
        let (x, y, w, h) = obstacle;
        if self.y >= y as i32 && self.y < (y + h) as i32 {
            if self.x <= x as i32 {
                return Place::Right;
            } else {
                return Place::Left;
            }
        } else if self.x >= x as i32 && self.x < (x + w) as i32 {
            if self.y <= y as i32 {
                return Place::Under;
            } else {
                return Place::Above;
            }
        } else {
            return Place::None;
        }
    }

    pub fn collision(&mut self) -> bool {
        if self.x < 0
            || self.y < 0
            || self.x > self.width as i32 - 1
            || self.y > self.height as i32 - 1
        {
            return true;
        }
        let x: u32 = self.x as u32;
        let y: u32 = self.y as u32;
        for &(obstacle_x, obstacle_y, obstacle_w, obstacle_h) in self.obstacles.iter() {
            if x >= obstacle_x
                && x < (obstacle_x + obstacle_w)
                && y >= obstacle_y
                && y < (obstacle_y + obstacle_h)
            {
                return true;
            } else {
                return false;
            }
        }
        false
    }

    pub fn get_state(&mut self) -> ([i32; 2], f64, [i32; 4]) {
        ([self.x, self.y], self.distance, self.obstacle_distance())
    }

    pub fn get_distance(&mut self) -> f64 {
        (((self.x - self.goal_x).pow(2) + (self.y - self.goal_y).pow(2)) as f64).sqrt()
    }
}

#[derive(Debug, EnumIter)]
pub enum Action {
    Up,
    Down,
    Right,
    Left,
}

enum Place {
    Above,
    Under,
    Left,
    Right,
    None,
}
