use crate::constants::{BEHAVIOUR_ENABLED, SCREEN_SIZE};
use crate::logic::boid::{Boid, BoidManager};
use crate::math::quadtree::quadt::QuadTree;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, WindowCanvas};
use sdl2::ttf::{self, FontStyle};
use sdl2::video::Window;

#[macro_use]
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub trait Renderable {
    fn render(&mut self, canvas: &mut WindowCanvas) -> Result<(), String>;
}

pub struct Writer<'ttf, 'b> {
    font: ttf::Font<'ttf, 'b>,
}

impl<'ttf, 'b> Writer<'ttf, 'b> {
    fn new(font: ttf::Font<'ttf, 'b>) -> Writer<'ttf, 'b> {
        Writer { font }
    }
}

pub struct GfxSubsystem<'ttf, 'b> {
    pub sdl_context: sdl2::Sdl,
    pub writer: Writer<'ttf, 'b>,
}

impl<'ttf, 'b> GfxSubsystem<'ttf, 'b> {
    pub fn new(ttf_context: &'ttf ttf::Sdl2TtfContext) -> GfxSubsystem<'ttf, 'b> {
        let sdl_context = sdl2::init().unwrap();
        let font = ttf_context.load_font("font.ttf", 128).unwrap();
        let mut writer = Writer::new(font);
        writer.font.set_style(FontStyle::BOLD);
        GfxSubsystem {
            sdl_context,
            writer,
        }
    }
}

pub struct RendererManager<'ttf, 'b> {
    canvas: WindowCanvas,
    gfx: GfxSubsystem<'ttf, 'b>,
}
impl<'ttf, 'b> RendererManager<'ttf, 'b> {
    pub fn new(
        window: Window,
        gfx: GfxSubsystem<'ttf, 'b>,
    ) -> Result<RendererManager<'ttf, 'b>, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(RendererManager { canvas, gfx })
    }
    pub fn draw(&mut self, boid_manager: &mut BoidManager) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        boid_manager.render(&mut self.canvas);
        /*
                for b in  boid_manager.boids{
                    b.draw_boid(&mut self.canvas)?;
                }
                quad_tree.render(&mut self.canvas)?;
        */

        unsafe {
            if !BEHAVIOUR_ENABLED.is_empty() {
                self.draw_string(BEHAVIOUR_ENABLED.to_string())?;
            } else {
                self.draw_string((&"NONE").to_string())?;
            }
        }

        self.canvas.present();
        Ok(())
    }
    pub fn draw_string(&mut self, text: String) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        // render a surface, and convert it to a texture bound to the canvas
        let surface = self
            .gfx
            .writer
            .font
            .render(&text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width, height, .. } = texture.query();

        let padding = 64;
        let target = self.get_centered_rect(
            width,
            height,
            SCREEN_SIZE.x - padding,
            SCREEN_SIZE.y - padding,
        );

        self.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
    fn get_centered_rect(
        &self,
        rect_width: u32,
        rect_height: u32,
        cons_width: u32,
        cons_height: u32,
    ) -> Rect {
        let wr = rect_width as f32 / cons_width as f32;
        let hr = rect_height as f32 / cons_height as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                let h = (rect_height as f32 / wr) as i32;
                (cons_width as i32, h)
            } else {
                let w = (rect_width as f32 / hr) as i32;
                (w, cons_height as i32)
            }
        } else {
            (rect_width as i32, rect_height as i32)
        };

        let cx = 0; //(SCREEN_SIZE.x as i32 - w) / 2;
        let cy = 0; //(SCREEN_SIZE.y as i32 - h) / 2;
        rect!(cx, cy, w, h)
    }
}
