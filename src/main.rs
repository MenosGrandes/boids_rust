pub mod constants;
pub mod graphics;
pub mod logic;
pub mod math;

extern crate sdl2;

use std::time::Duration;

use constants::SCREEN_SIZE;
use graphics::renderer::{GfxSubsystem, RendererManager};
use logic::boid::BoidManager;
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;

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
    let mut boid_manager = BoidManager::new();
    boid_manager.spawn_boid(20);
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
                        boid_manager.spawn_boid(10);
                    }
                    Keycode::R => {
                        boid_manager.remove_all_boids();
                        boid_manager.spawn_boid(10);
                    }
                    Keycode::Escape => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }
        renderer.draw_string(fps_manager.get_framerate().to_string())?;
        renderer.draw(&boid_manager.boids)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps_manager.get_framerate() as u32));
        boid_manager.update_all();

        // The rest of the game loop goes here...
    }

    Ok(())
}
