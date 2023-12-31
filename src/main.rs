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
    BehaviourEnabled, DrawPrimitives, BEHAVIOUR_ENABLED, BOIDS_AMOUNT, BORDER_BEHAVIOUR,
    DRAW_PRIMITIVES, SCREEN_SIZE, VIEW_PORT_SIZE,
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

pub fn main() -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().unwrap();
    let gss = GfxSubsystem::new(&ttf_context);

    let video_subsystem = gss.sdl_context.video()?;
    let window = video_subsystem
        .window("Boids", SCREEN_SIZE.x as u32, SCREEN_SIZE.y as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = gss.sdl_context.event_pump()?;
    let mut renderer = RendererManager::new(window, gss);

    let mut fps_manager: FPSManager = FPSManager::new();
    fps_manager.set_framerate(100)?;

    let r: Region = Region::new(Vector2::new(0.0, 0.0), VIEW_PORT_SIZE);
    let mut boid_manager = BoidManager::new(r.clone());
    boid_manager.spawn_boid(BOIDS_AMOUNT);

    let mut camera = camera::Camera::new(r.left_up);
    log::info!("camera position {:?}", camera);

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

                    Keycode::Num4 => unsafe {
                        BEHAVIOUR_ENABLED ^= BehaviourEnabled::BOUND;
                    },
                    Keycode::Num5 => {
                        DRAW_PRIMITIVES.with(|value| {
                            *value.borrow_mut() ^= DrawPrimitives::QUAD_TREE;
                        });
                    }
                    Keycode::Num6 => {
                        DRAW_PRIMITIVES.with(|value| {
                            *value.borrow_mut() ^= DrawPrimitives::BOID_VIEW;
                        });
                    }
                    Keycode::Num7 => {
                        DRAW_PRIMITIVES.with(|value| {
                            *value.borrow_mut() ^= DrawPrimitives::BOUND_VIEW;
                        });
                    }
                    Keycode::Left => {
                        camera.pos.x -= 20.0;
                    }
                    Keycode::Right => {
                        camera.pos.x += 20.0;
                    }
                    Keycode::Down => {
                        camera.pos.y += 20.0;
                    }
                    Keycode::Up => {
                        camera.pos.y -= 20.0;
                    }
                    Keycode::Escape => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }
        renderer.draw(&mut boid_manager, &camera);
        boid_manager.update();
        ::std::thread::sleep(Duration::new(
            0,
            1_000_000_000u32 / fps_manager.get_framerate() as u32,
        ));
    }

    Ok(())
}
