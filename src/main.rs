/*pub mod constants;
pub mod graphics;
pub mod logic;
pub mod math;

extern crate approx;
extern crate sdl2;
extern crate imgui;
extern crate imgui_sdl2;

use std::time::Duration;

use constants::{
    BehaviourEnabled, BEHAVIOUR_ENABLED, BOIDS_AMOUNT, BORDER_BEHAVIOUR, DRAW_VIEW, SCREEN_SIZE,
};
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
    /*Init Logger*/
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d}: {l} - {m}\n")))
        .build("log/output.log");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile.unwrap())))
        .build(Root::builder().appender("logfile").build(LevelFilter::Error));

    let _ = log4rs::init_config(config.unwrap());

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
    fps_manager.set_framerate(100)?;

    let r: Region = Region::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32),
    );
    let mut boid_manager = BoidManager::new(r);
    boid_manager.spawn_boid(BOIDS_AMOUNT);
    let mut event_pump = gss.sdl_context.event_pump()?;
    let mut renderer = RendererManager::new(window, gss);

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
        boid_manager.update();
        renderer.draw(&mut boid_manager);
        ::std::thread::sleep(Duration::new(
            0,
            1_000_000_000u32 / fps_manager.get_framerate() as u32,
        ));
    }

    Ok(())
}
*/
extern crate sdl2;
extern crate imgui;
extern crate imgui_sdl2;
extern crate gl;
extern crate imgui_opengl_renderer;

use std::time::Instant;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video = sdl_context.video().unwrap();

  {
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
  }

  let window = video.window("rust-imgui-sdl2 demo", 1000, 1000)
    .position_centered()
    .resizable()
    .opengl()
    .allow_highdpi()
    .build()
    .unwrap();

  let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
  gl::load_with(|s| video.gl_get_proc_address(s) as _);

  let mut imgui = imgui::Context::create();
  imgui.set_ini_filename(None);


  let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

  let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

  let mut event_pump = sdl_context.event_pump().unwrap();

  let mut last_frame = Instant::now();


  'running: loop {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    for event in event_pump.poll_iter() {
      imgui_sdl2.handle_event(&mut imgui, &event);
      if imgui_sdl2.ignore_event(&event) { continue; }

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}
      }
    }


    imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

    let now = Instant::now();
    let delta = now - last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    last_frame = now;
    imgui.io_mut().delta_time = delta_s;

    let ui = imgui.frame();
    ui.show_demo_window(&mut true);

    unsafe {
      gl::ClearColor(0.2, 0.2, 0.2, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    imgui_sdl2.prepare_render(&ui, &window);
    renderer.render(ui);

    window.gl_swap_window();

    ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
  }
}

