use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;


use std::time::SystemTime;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Dir {
    up,
    right,
    down,
    left,
}

impl Dir {
    fn right(self) -> Dir {
        match self {
            Dir::up => {Dir::right},
            Dir::right => {Dir::down},
            Dir::down => {Dir::left},
            Dir::left => {Dir::up},
        }
    }
    fn left(self) -> Dir {
        match self {
            Dir::up => {Dir::left},
            Dir::left => {Dir::down},
            Dir::down => {Dir::right},
            Dir::right => {Dir::up},
        }
    }
    fn as_vec(self) -> (i32, i32) {
        match self {
            Dir::up => {(0, -1)},
            Dir::down => {(0, 1)},
            Dir::left => {(-1, 0)},
            Dir::right => {(1, 0)},
        }
    }
}

#[derive(Default, Debug)]
struct InfiniteGrid {
    low_bound: (i32, i32),
    high_bound: (i32, i32),
    entries: HashMap<(i32, i32), i32>,
}

impl InfiniteGrid {
    fn new(start_size: i32) {

    }

    fn set(&mut self, x: i32, y: i32, val: i32) {
        self.low_bound.0 = self.low_bound.0.min(x);
        self.low_bound.1 = self.low_bound.1.min(y);
        self.high_bound.0 = self.high_bound.0.max(x);
        self.high_bound.1 = self.high_bound.1.max(y);
        self.entries.insert((x, y), val);
    }

    fn get(&self, x: i32, y: i32) -> i32 {
        *self.entries.get(&(x, y)).unwrap_or(&0)
    }

    fn size(&self) -> (i32, i32){
        (self.high_bound.0 - self.low_bound.0, self.high_bound.1 - self.low_bound.1)
    }

    fn longest_size(&self) -> i32 {
        self.size().0.max(self.size().1)
    }
}

#[derive(Debug)]
struct SimState {
    rule_str: &'static str,
    grid: InfiniteGrid,
    ant_pos: (i32, i32),
    ant_dir: Dir,
}

impl SimState {
    fn update(&mut self) {
        let (x, y) = self.ant_pos;
        let rule_idx = self.grid.get(x, y);
        self.ant_dir = match self.rule_str.chars().nth(rule_idx as usize).unwrap() {
            'R' => {self.ant_dir.right()},
            'L' => {self.ant_dir.left()},
            'S' => {self.ant_dir},
            _ => {panic!("bad rule str")},
        };
        let (dx, dy) = self.ant_dir.as_vec();
        self.grid.set(x, y, (rule_idx + 1) % self.rule_str.len() as i32);
        self.ant_pos = (x + dx, y + dy);
    }

    fn draw(&self, res: u32, canvas: &mut Canvas<Window>, painter_fn: fn(&SimState, i32) -> Color) {
        canvas.set_draw_color(painter_fn(self, 0));
        canvas.clear();

        let grid_start = self.grid.low_bound;
        let grid_size = self.grid.longest_size();

        let tile_px = res / grid_size as u32;

        for ((x, y), value) in self.grid.entries.iter() {
            let sx = x - grid_start.0;
            let sy = y - grid_start.1;
            let tile_rect = sdl2::rect::Rect::new(sx*tile_px as i32, sy*tile_px as i32, tile_px as u32, tile_px as u32);
            canvas.set_draw_color(painter_fn(self, *value));
            canvas.fill_rect(tile_rect).unwrap();
        }
    }
}

fn colour_vom(s: &SimState, i: i32) -> Color {
    let colours = vec![
        Color::RGB(255,255,255),
        Color::RGB(0,0,0),
        Color::RGB(255,0,0),
        Color::RGB(255,255,0),
        Color::RGB(0,255,0),
        Color::RGB(0,255,255),
        Color::RGB(0,0,255),
        Color::RGB(255,0,255),
        Color::RGB(128,0,0),
        Color::RGB(128,128,0),
        Color::RGB(0,128,0),
        Color::RGB(0,128,128),
        Color::RGB(0,0,128),
    ];

    return colours[i as usize % colours.len()];
}

fn colour_grey(s: &SimState, i: i32) -> Color {
    let colour_amt = (((i) * 255) / (s.rule_str.len()-1) as i32) as u8;
    Color::RGB(colour_amt, colour_amt, colour_amt)
}

fn colour_rl (s: &SimState, i: i32) -> Color {
    let colour_amt = ((i * 255) / s.rule_str.len() as i32) as u8;
    if s.rule_str.chars().nth(i as usize).unwrap() == 'R' {
        Color::RGB(colour_amt as u8, 0, 0)
    } else {
        Color::RGB(0, 0, colour_amt as u8)
    }
} 

fn main() {

    let res = 800;

    // get some 180s around
    // other ca rock paper scissors

// convoluted highway LLRRRLRLRLLR
// is there some condition necessary for a highway???
// like number of Rs and Ls

    // usability eg change rule, next rule, etc

    // ah yes N and U is expanded in turmites
    // ah turmites is the ant has a state too, its like a turing machine
    // CA environments 

    // what if it spawned other ants and they would die
    // one could be spawns and one could be dies
    // RRLLRPD

    // could spawn again at origin after every death
    // dies and respawns at origin could be cool
    // genetic algorithm to train rules

    // write a function that tells you what it ends up as, e.g. spaceship of period N

    // turn around??

    // crystal defects could make it interesting: very low probability of doing the wrong thing

    // symmetric rules???
    // what kind of growrth rate, growth guaranted to continue/ any clocks?

    // also histogram of distribution of each pixel
    // how spherical

    // how about does rotating the sequence do anything. nope it would not
    // spin them all up side by side

    // ah max size and follow the ant ?

    // go straight rule?
    // trace the outline of one rotation of RLLR hey

    // ah yes a heuristic to automate the search,
    // one of fuck noise and maybe fuck highways? well highways are cool
    // greatest area visited per distance maybe

    // all the possible ways to go from one rule to another
    // repeat changing 1 thing
    // add stuff on end
    // cascade

    // spaceship of increasing period - triangles and squares

    let mut state = SimState {
        //rule_str: "RL",
        //rule_str: "LRRRRRLLR", // square one // can make the RRRRR arbitrarily wrong
        //rule_str: "RRLLLRLLLRRR", // triangle
        //rule_str: "RRLLLRLLLRRRRR", // cool as fuck, one less R cool too
        //rule_str: "RRLLLRLLLRRRRRRRRL", // triangle
        //rule_str: "RRLLLRLLLRRLLRRRRRLLR", // frost
        //rule_str: "RLLR", // symmetrical square
        //rule_str: "RLLR", // symmetricla square
        //rule_str: "LLRR", // symmetrical
        //rule_str: "RRLLLLLLR", // chaotic crystals // yea figuring it out sick
        //rule_str: "RRRLRRLLRLRRRRLLRRLRLRRR", // binary counter, meant to be at least from reddit
        //rule_str: "RLL", // cave generator
        //rule_str: "RRRLLLL", // more round cause RRR start
        //rule_str: "RRRLLLLR", // wow symmetry. whats the property that gives symmetry? num R = num L ? not quite. no single Rs and Ls? even wrt wrap
        //rule_str: "RRLLRRLLRRLLRRLLRRLL", // brain
        //rule_str: "RLRRLLLRRRRRLLLLLLLLRRRRRRRRRRRRRLLLLLLLLLLLLLLLLLLLLLRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
        //rule_str: "RLRRLLLRRRRRLLLLLLLLRRRRRLLLRRLR",
        //rule_str: "RLRRLLLRRRRRLLLRRLR",
        //rule_str: "RLLRRRRLLLLLLLLRRRRRRRRRRRRRRRR", // very slow expanding lol
        //rule_str: "RLLRRRRLLL", // 3d one less L wall
        // pascals triangle, odd and even
        // theres a lot of possible sequences
        //rule_str: "LRLRRLRRRLRRRRLRRRRRLRRRRRRLRRRRRRRLRRRRRRRRLRRRRRRRRRLRRRRRRRRRRLRRRRRRRRRRRLRRRRRRRRRRRR",    // cool if you make this arbitrarily long i think its a fractal // lol S on this one space ships, only one that isnt a blob

        // if you train it not to overflow, you just add L or R on the end and see which one overflows less,
        // could find out rules

        // dragon curve?
        // generate these with l systems
        // this is the dragon curve one
        // every dragon curve spaceships?????
        //rule_str: "RRLRRLRRLL",



        //rule_str:"RSRRSRRRS", // omg it counts
        //rule_str:"RRS", // omg it counts
        //rule_str:"LSSSSS", // more S more gradient. why camera wrong?
        //rule_str:"RSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSL", // sick crystal thing
        //rule_str:"RSSSSSSSSRRRSS", // sick crystal thing // cool familr
        //rule_str:"RLS", // spaceship
        rule_str:"RRLRR", //square ratio 4:1?? nah
        // rule_str:"RRLRRRRLRRR", // cool augment square a bit, dopuble + extra r
        
        // truncating the series causes noise patches and shit, breaks the structure.
        // so similarly when theres structure and then it gives way to noise ask 'what was the series meant to be?'
        //rule_str:"RRLLLRLLLL", // eg wrt this one, which has sierpinski triangles
        




        // what is it about it that makes a square, how to have an arbitrary payload for the square?
        // how to program it lol

        // RSSS... family, or RSSS..RS..RS.... family
        
        // primes brah
        //rule_str: "RRLLLRRRRRLLLLLLLRRRRRRRRRRLLLLLLLLLLLLLRRRRRRRRRRRRRRRRRLLLLLLLLLLLLLLLLLLL",
        //rule_str: "RRLRRLLRRRLLRLL",
        // 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97.

        //rule_str:"RRRLRL", // polyrhythm sequence, like RLL
        //rule_str: "LLRRRLRLRL", // kind of aggressively spikey, wanting to spaceship i guess
        //rule_str: "LLRRRL",


        //rule_str: "LRRRRLLLLLLRRRRL",
        grid: InfiniteGrid::default(),
        ant_pos: (0,0),
        ant_dir: Dir::up,
    };
    // matters what you start with doesnt it

    // wrap with mod
    // also grayscale would be quite informative
    // wanna look at some long ones
    // grayscale + colour for R or L !
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("ANT", res, res)
        .position_centered()
        .build()
        .expect("failed making window");

    let mut canvas = window.into_canvas().build()
        .expect("couldnt make canvas");

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut dt = 1.0f64 / 60f64;

    'running: loop {
        let loop_start = SystemTime::now();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                }
                _ => {}
            }
        }
        
        // update
        for _ in 1..10 {
            state.update();
        }


        // draw
        state.draw(res, &mut canvas, colour_grey);


        canvas.present();

        let loop_end = SystemTime::now();
        let delta = loop_end.duration_since(loop_start).unwrap().as_secs_f64();
        let frame_cap = 1.0 / 60.0;
        if delta < frame_cap {
            std::thread::sleep(Duration::from_secs_f64(frame_cap - delta));
            dt = frame_cap;
        } else {
            dt = delta;
        }
        //println!("{} fps", 1.0/dt);
    }
}
