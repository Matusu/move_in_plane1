struct Enviroment{
    width: u32,
    height: u32,
    begin_x: i32,
    begin_y: i32,
    x: i32,
    y: i32,
    goal_x: i32,
    goal_y: i32,
    distance: f64,
    obstacle: [u32;4],
}

impl Enviroment {
    fn new(width: u32, height: u32, begin_x: i32, begin_y: i32, goal_x: i32, goal_y: i32) -> Self{
        Enviroment {width,height,begin_x,begin_y,x:begin_x,y:begin_y,goal_x,goal_y,
            distance: (((begin_x - goal_x).pow(2) + (begin_y - goal_y).pow(2)) as f64).sqrt(),
            obstacle: [0,0,0,0]
        }
    }

    fn reset(&mut self) {
        self.x = self.begin_x;
        self.y = self.begin_y;
    }
    fn step(&mut self,action: Action) -> i32{
        let mut reward: i32 = 0;
        match action {
            Action::Up => if self.y - 1 < 0 {return -10} else {self.y -= 1;},
            Action::Down => if self.y + 1 > (self.height as i32) -1 {return -10} else {self.y += 1;},
            Action::Left => if self.x -1 < 0 {return -10} else {self.x -=1},
            Action::Right => if self.x + 1 > (self.height as i32) -1 {return -10} else {self.x+=1},
        }
        let new_distance = self.get_distance();
        let compare: bool = self.distance > new_distance;
        self.distance = new_distance;
        if compare{
            reward-1
        }
        else{
            reward + 1
        }
    }

    fn done(&mut self,xy:i32,axes:Axes) -> bool{
        let boundadries = match axes{
            Axes::X => self.width-1,
            Axes::Y => self.height-1,
        };
        if xy > boundadries as i32{
            true
        }
        else if xy < 0{
            true
        }
        else{
            false
        }
    }

    fn get_state(&mut self) -> ([i32;2],f64){
        ([self.x,self.y],self.distance)
    }

    fn get_distance(&mut self) ->f64{
        (((self.x - self.goal_x).pow(2) + (self.y - self.goal_y).pow(2)) as f64).sqrt()
    }
}

enum Axes{
    X,
    Y,
}

enum Action {
    Up,
    Down,
    Right,
    Left
}

fn main() {
    let mut env = Enviroment::new(10,10,0,0,5,5);
    let (position,distance) = env.get_state();
    for i in position{
        print!("{i}");
    }
    println!("{distance}");
    for _ in 1..=5{
        env.step(Action::Down);
    let (position,distance) = env.get_state();
    for i in position{
        print!("{i}");
    }
    println!("{distance}");
    }
}
