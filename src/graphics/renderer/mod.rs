use crate::logic::boid::Boid;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render:: WindowCanvas;
use sdl2::ttf;
use sdl2::video::Window;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub trait Renderable {
    fn render(&mut self, canvas: &WindowCanvas) -> Result<() , String>;
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
        let font = ttf_context.load_font("font.ttf", 24).unwrap();
        let writer = Writer::new(font);
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
    pub fn draw(&mut self, boids: &Vec<Boid>) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        for b in boids {
            b.draw_boid(&self.canvas)?;
        }
        self.canvas.present();

        Ok(())
    }
    pub fn draw_string(&mut self, text: String) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        //

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
        //let TextureQuery { width, height, .. } = texture.query();

        let target = self. get_upper_rect(64,64);

        self.canvas.copy(&texture, None, Some(target))?;
        self.canvas.present();
        Ok(())
    }
    fn get_upper_rect(
        &self,
        rect_width: u32,
        rect_height: u32) ->Rect
    {
        rect!(0,0, rect_height,rect_width)
    }

}
