pub mod camera;
pub mod constants;
pub mod ecs;
pub mod game;
pub mod graphics;
pub mod logic;
pub mod math;

extern crate approx;
extern crate crossbeam;
extern crate sdl2;

use std::time::Duration;

use constants::{
    BehaviourConsts, BehaviourEnabled, DrawPrimitives, BEHAVIOUR_ENABLED, BOIDS_AMOUNT,
    BORDER_BEHAVIOUR, DRAW_PRIMITIVES, MAX_BOID_SPEED, SCREEN_SIZE, VIEW_PORT_SIZE,
};
use game::GameBuilder;
use graphics::renderer::{GfxSubsystem, RendererManager};
use logic::behaviour::traits::BorderBehaviourE;
use logic::boid::boid_mgr::BoidManager;
use logic::boid::traits::Updatable;
use math::quadtree::region::Region;
use math::vec::Vector2;
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use specs::{
    Builder, Component, DispatcherBuilder, Entities, ReadStorage, System, VecStorage, World,
    WorldExt, WriteStorage,
};

use specs::Join;

use crate::math::vec::{Distance, Magnitude, V2f32};
#[derive(Debug)]
struct Position {
    v: V2f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    v: V2f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct AvgVelocity {
    v: V2f32,
}

impl Component for AvgVelocity {
    type Storage = VecStorage<Self>;
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        ReadStorage<'a, AvgVelocity>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (avg_vel, mut vel, mut pos): Self::SystemData) {
        for (avg_vel, vel, pos) in (&avg_vel, &mut vel, &mut pos).join() {
            pos.v += vel.v;
            vel.v += avg_vel.v;
            vel.v.limit(MAX_BOID_SPEED)
        }
    }
}

struct AvgVelocitySystem;

impl<'a> System<'a> for AvgVelocitySystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, AvgVelocity>,
    );

    fn run(&mut self, (entities, vel, position, mut avg_vel): Self::SystemData) {
        for e in entities.join() {
            let avg_vel_e = avg_vel.get_mut(e);
            if let Some(avg_vel_e) = avg_vel_e {
                for vel in vel.join() {
                    avg_vel_e.v += vel.v;
                }
                avg_vel_e.v.set_magnitude(MAX_BOID_SPEED);
                avg_vel_e.v -= vel.get(e).unwrap().v;
                avg_vel_e.v *= BehaviourConsts::ALLIGN_FACTOR;
            };
        }
    }
}
struct AllignSystem;

impl<'a> System<'a> for AllignSystem {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.v += vel.v * 0.05;
        }
    }
}
// Type alias for the data needed by the renderer
pub type SystemDataDraw<'a> = ReadStorage<'a, Position>;
fn draw(canvas: &mut WindowCanvas, data: SystemDataDraw) {
    canvas.set_draw_color(Color::GREEN);
    canvas.clear();
    for position in data.join() {
        let p: Point = Point::new(position.v.x as i32, position.v.y as i32);
        let r = Rect::from_center(p, 2, 2);
        let _ = canvas.rectangle(
            r.top_left().x as i16,
            r.top_left().y as i16,
            r.bottom_right().x as i16,
            r.bottom_right().y as i16,
            Color::BLUE,
        );
    }
    canvas.present();
}
use sdl2::pixels::Color;
fn build_boid(world: &mut World) {
    let mut c = Vector2::random(-0.5, 0.5);
    c.set_magnitude(10.0);
    let rand_pos =
        Vector2::random_from_vec(Vector2::new(0.0, 800 as f32), Vector2::new(0.0, 600 as f32));
    world
        .create_entity()
        .with(Position { v: rand_pos })
        .with(Velocity { v: c })
        .with(AvgVelocity { v: V2f32::zero() })
        .build();
}

pub fn main() -> Result<(), String> {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<AvgVelocity>();

    for _ in 0..1000 {
        build_boid(&mut world);
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(AvgVelocitySystem, "allign", &[])
        .with(UpdatePos, "update_pos", &["allign"])
        .build();

    /*
     * DRAW!
     * */
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        //canvas.present();
        // The rest of the game loop goes here...

        dispatcher.dispatch(&mut world);
        world.maintain();
        draw(&mut canvas, world.system_data());
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
