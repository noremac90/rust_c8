use c8::Chip8;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image, DrawParam};
use ggez::event::{self, EventHandler};

mod c8;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_setup(ggez::conf::WindowSetup { title: "STUFF".into(), samples: ggez::conf::NumSamples::Sixteen, vsync: false, icon: "".into(), srgb: false })
        .window_mode(ggez::conf::WindowMode { width: 800.0, height: 400.0, maximized: false, fullscreen_type: ggez::conf::FullscreenType::Windowed, borderless: false, min_width: 0.0, min_height: 0.0, max_width: 0.0, max_height: 0.0, resizable: true, visible: true, resize_on_scale_factor_change: false })
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
    cpu: c8::Chip8
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.

        graphics::set_default_filter(_ctx, graphics::FilterMode::Nearest);
        let m = MyGame {
            cpu: Chip8::load("ibm.ch8")
        };

        
        m
    }
}

impl EventHandler for MyGame {

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.cpu.execute();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        
        let mut bytes =[0; 2048 * 4];

        for y in 0..32 {
            for x in 0..64 {
                let i = x + y * 64;
                if self.cpu.vram[i] == 1 {
                    bytes[i * 4] = 255;
                    bytes[i * 4 + 1] = 255;
                    bytes[i * 4 + 2] = 255;
                    bytes[i * 4 + 3] = 255;
                }
            }
        }
        

        let img = Image::from_rgba8(ctx, 64, 32, &bytes)?;
        

        let (wx, wy) = graphics::drawable_size(ctx);

        graphics::draw(ctx, &img, DrawParam::default().scale([wx / 64.0, wy / 32.0]))?;


        let text = graphics::Text::new(format!("{}", ggez::timer::fps(ctx)));

        graphics::draw(ctx, &text, DrawParam::default())?;

        graphics::present(ctx)
    }
}