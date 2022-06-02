use chrono::{Duration, NaiveTime, Utc};
use rand::Rng;
use raylib::prelude::*;
use std::ffi::CString;

const WINDOW_WIDTH: i32 = 860;
const WINDOW_HEIGHT: i32 = 660;
const MAX_FPS_TARGET: u32 = 60;

const MIN_SPEED: f32 = -1.0;
const MAX_SPEED: f32 = 2.0;

const CONTAMINATION_RADIUS: f32 = 5.0;
const TOTAL_POPULATION_SIZE: i32 = 600;
const INITIAL_INFECTED_POPULATION: i32 = 20;
const INFECTION_RATE: f32 = 0.02;
const DEATH_RATE: f32 = 0.01;
const RECOVERY_RATE: f32 = 0.98;

#[derive(Debug, Copy, Clone)]
struct Human {
    pos: Vector2,
    vel: Vector2,
    infected: bool,
    infected_at: chrono::naive::NaiveTime,
}

impl Human {
    fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32, infected: bool) -> Human {
        let mut rng = rand::thread_rng();

        let pos_x: f32 = rng.gen_range(start_x..end_x) as f32;
        let pos_y: f32 = rng.gen_range(start_y..end_y) as f32;

        let vel_x: f32 = rng.gen_range(MIN_SPEED..MAX_SPEED) as f32;
        let vel_y: f32 = rng.gen_range(MIN_SPEED..MAX_SPEED) as f32;

        Human {
            pos: Vector2::new(pos_x, pos_y),
            vel: Vector2::new(vel_x, vel_y),
            infected: infected,
            infected_at: NaiveTime::from_hms_milli(0, 0, 0, 0),
        }
    }

    fn update_position(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    fn wall_bump(&mut self) {
        if self.pos.x < 0.0 || self.pos.x > WINDOW_WIDTH as f32 {
            self.vel.x *= -1.0;
        }
        if self.pos.y < 0.0 || self.pos.y > WINDOW_HEIGHT as f32 {
            self.vel.y *= -1.0;
        }
    }

    fn populate(healthy: &mut Vec<Human>, infected: &mut Vec<Human>) {
        for _i in 0..TOTAL_POPULATION_SIZE - INITIAL_INFECTED_POPULATION {
            let new_healthy = Human::new(1, 1, WINDOW_WIDTH, WINDOW_HEIGHT, false);
            healthy.push(new_healthy);
        }

        for _i in 0..INITIAL_INFECTED_POPULATION {
            let new_infected = Human::new(1, 1, WINDOW_WIDTH, WINDOW_HEIGHT, true);
            infected.push(new_infected);
        }
    }

    fn contaminate(
        healthy: &mut Vec<Human>,
        infected: &mut Vec<Human>,
        infected_idx: i32,
        mut infected_end_idx: i32,
    ) -> i32 {
        let mut j = 0;
        let mut end_j = healthy.len() as i32;

        while j < end_j {
            if collision::check_collision_circles(
                infected[infected_idx as usize].pos,
                CONTAMINATION_RADIUS,
                healthy[j as usize].pos,
                CONTAMINATION_RADIUS,
            ) {
                let random = rand::thread_rng().gen_range(1..100) as f32 / 100.0;

                if random <= INFECTION_RATE {
                    let mut new_infected = healthy[j as usize];
                    new_infected.infected = true;
                    new_infected.infected_at = Utc::now().time();

                    infected.push(new_infected);
                    healthy.remove(j as usize);

                    end_j -= 1;
                    infected_end_idx -= 1;
                }
            }
            j += 1;
        }

        return infected_end_idx;
    }

    fn will_die(&mut self) -> bool {
        let random = rand::thread_rng().gen_range(1..100) as f32 / 100.0;

        if random <= DEATH_RATE {
            return true;
        }
        return false;
    }

    fn will_recover(&mut self) -> bool {
        let random = rand::thread_rng().gen_range(1..100) as f32 / 100.0;

        if random >= RECOVERY_RATE {
            return true;
        }

        return false;
    }

    fn simulate(
        healthy: &mut Vec<Human>,
        infected: &mut Vec<Human>,
        recovered: &mut Vec<Human>,
        dead: &mut Vec<Human>,
        infected_idx: i32,
        mut infected_end_idx: i32,
    ) -> i32 {
        infected_end_idx = Human::contaminate(healthy, infected, infected_idx, infected_end_idx);

        let incubation_period = Utc::now().time() - infected[infected_idx as usize].infected_at;

        if incubation_period >= Duration::seconds(20) {
            if infected[infected_idx as usize].will_die() {
                dead.push(infected[infected_idx as usize]);
                infected.remove(infected_idx as usize);
                infected_end_idx -= 1;
            } else if infected[infected_idx as usize].will_recover() {
                recovered.push(infected[infected_idx as usize]);
                infected.remove(infected_idx as usize);
                infected_end_idx -= 1;
            }
        }

        return infected_end_idx;
    }
}

fn main() {
    let mut healthy: Vec<Human> = Vec::new();
    let mut infected: Vec<Human> = Vec::new();
    let mut recovered: Vec<Human> = Vec::new();
    let mut dead: Vec<Human> = Vec::new();

    Human::populate(&mut healthy, &mut infected);

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Simple Pandemic Simulator")
        .build();
    rl.set_target_fps(MAX_FPS_TARGET);

    let mut play: bool = true;
    let mut reset: bool;

    let play_rect = Rectangle {
        x: 10.0,
        y: 10.0,
        width: 40.0,
        height: 40.0,
    };
    let reset_rect = Rectangle {
        x: 60.0,
        y: 10.0,
        width: 40.0,
        height: 40.0,
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_fps(WINDOW_WIDTH - 90, 10);

        if play {
            play = d.gui_toggle(
                play_rect,
                Some(CString::new("Pause".as_bytes()).unwrap().as_c_str()),
                play,
            );
        } else {
            play = d.gui_toggle(
                play_rect,
                Some(CString::new("Play".as_bytes()).unwrap().as_c_str()),
                play,
            );
        }

        reset = d.gui_button(
            reset_rect,
            Some(CString::new("Reset".as_bytes()).unwrap().as_c_str()),
        );

        if reset {
            infected.clear();
            healthy.clear();
            recovered.clear();
            dead.clear();

            Human::populate(&mut healthy, &mut infected);
        }

        let mut infected_idx: i32 = 0;
        let mut infected_end_idx: i32 = infected.len() as i32;

        while infected_idx < infected_end_idx {
            d.draw_circle_v(infected[infected_idx as usize].pos, 5.0, Color::RED);

            if play {
                infected[infected_idx as usize].update_position();
                infected[infected_idx as usize].wall_bump();

                infected_end_idx = Human::simulate(
                    &mut healthy,
                    &mut infected,
                    &mut recovered,
                    &mut dead,
                    infected_idx,
                    infected_end_idx,
                );

                std::println!(
                    "infected: {} ; healthy: {} ; recovered: {} ; dead: {}",
                    infected.len(),
                    healthy.len(),
                    recovered.len(),
                    dead.len()
                );
            }

            infected_idx += 1;
        }

        for human in healthy.iter_mut() {
            d.draw_circle_v(human.pos, 5.0, Color::BLUE);

            if play {
                human.update_position();
                human.wall_bump();
            }
        }

        for human in recovered.iter_mut() {
            d.draw_circle_v(human.pos, 5.0, Color::PURPLE);

            if play {
                human.update_position();
                human.wall_bump();
            }
        }

        for human in dead.iter_mut() {
            d.draw_circle_v(human.pos, 5.0, Color::BLACK);
        }
    }
}
