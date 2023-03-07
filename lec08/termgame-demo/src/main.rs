use bmp::{Image, Pixel};
use termgame::{SimpleEvent, Controller, Game, GameEvent, GameSettings, StyledCharacter, run_game, KeyCode, GameColor, GameStyle};
use std::error::Error;
use std::time::Duration;

const BORDER_WIDTH: u16 = 2;

struct MyGame {
    image: Image,
}

impl Controller for MyGame {
    fn on_start(&mut self, game: &mut Game) {
        let (width, (height, _)) = game.screen_size();

        let (term_width, term_height) = (width - BORDER_WIDTH, height - BORDER_WIDTH);
        let (image_width, image_height) = (dbg!(self.image.get_width()), dbg!(self.image.get_height()));

        // Design limitation: assumption that image dimensions > term dimensions

        for x in 0..term_width {
            for y in 0..term_height {

                // bug from lecture (reported by Xavier)
                // the integer division rounded the edge of the image 
                // away, so we do the multiplication first to reduce
                // the overall rounding loss!

                let Pixel { r, g, b } = self.image.get_pixel(
                    image_width * x as u32 / term_width as u32,
                    image_height * y as u32 / term_height as u32,
                );

                game.set_screen_char(
                    x as _,
                    y as _,
                    Some(StyledCharacter::new(' ')
                         .style(GameStyle::new()
                                .background_color(Some(GameColor::Rgb(r, g, b))))
                    )
                );
            }
        }
    }

    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        // match event.into() {
        //     SimpleEvent::Just(KeyCode::Char(ch)) => {
        //         game.set_screen_char(1, 1, Some(StyledCharacter::new(ch)))
        //     },
        //     _ => {}
        // }

    }

    fn on_tick(&mut self, _game: &mut Game) {
        
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let image = bmp::open("image.bmp").unwrap();

    let mut controller = MyGame {
        image,
    };

    run_game(
        &mut controller,
        GameSettings::new()
            // The below are the defaults, but shown so you can edit them.
            .tick_duration(Duration::from_millis(50))
            .quit_event(Some(SimpleEvent::WithControl(KeyCode::Char('c')).into()))
    )?;

    println!("Game Ended!");

    Ok(())
}
