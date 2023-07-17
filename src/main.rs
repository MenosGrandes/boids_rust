pub mod constants;
pub mod graphics;
pub mod logic;
pub mod math;

extern crate approx;
extern crate crossbeam;
extern crate sdl2;

use std::time::Duration;

use constants::{
    BehaviourEnabled, BEHAVIOUR_ENABLED, BOIDS_AMOUNT, BORDER_BEHAVIOUR, DRAW_VIEW, SCREEN_SIZE,
};
use graphics::renderer::{GfxSubsystem, RendererManager};
use logic::behaviour::traits::BorderBehaviourE;
use logic::boid::boid_mgr::BoidManager;
use math::quadtree::region::Region;
use math::vec::{V2usize, Vector2};
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;

use crate::logic::boid::traits::Updatable;

pub fn main() -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().unwrap();
    let gss = GfxSubsystem::new(&ttf_context);

    let video_subsystem = gss.sdl_context.video()?;
    let window = video_subsystem
        .window("Boids", SCREEN_SIZE.x, SCREEN_SIZE.y)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut fps_manager: FPSManager = FPSManager::new();
    fps_manager.set_framerate(120)?;
    let r: Region = Region::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32),
    );
    let mut boid_manager = BoidManager::new(r);
    boid_manager.spawn_boid(BOIDS_AMOUNT);
    let mut event_pump = gss.sdl_context.event_pump()?;
    let mut renderer = RendererManager::new(window, gss)?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => {
                        boid_manager.add_boid(1);
                    }
                    Keycode::D => {
                        DRAW_VIEW.with(|value: &std::cell::RefCell<bool>| {
                            let v = match *value.borrow() {
                                true => false,
                                false => true,
                            };
                            *value.borrow_mut() = v;
                        });
                    }
                    Keycode::R => {
                        BORDER_BEHAVIOUR.with(|value: &std::cell::RefCell<BorderBehaviourE>| {
                            let v = match *value.borrow() {
                                BorderBehaviourE::GoThrough => BorderBehaviourE::Reflect,
                                BorderBehaviourE::Reflect => BorderBehaviourE::GoThrough,
                            };
                            *value.borrow_mut() = v;
                        });
                    }
                    Keycode::Num1 => unsafe {
                        BEHAVIOUR_ENABLED ^= BehaviourEnabled::COHESION;
                    },
                    Keycode::Num2 => unsafe {
                        BEHAVIOUR_ENABLED ^= BehaviourEnabled::ALLIGN;
                    },
                    Keycode::Num3 => unsafe {
                        BEHAVIOUR_ENABLED ^= BehaviourEnabled::SEPERATE;
                    },
                    Keycode::Escape => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }
        renderer.draw(&mut boid_manager)?;
        ::std::thread::sleep(Duration::new(
            0,
            1_000_000_000u32 / fps_manager.get_framerate() as u32,
        ));
        //boid_manager.update();
    }

    Ok(())
}
