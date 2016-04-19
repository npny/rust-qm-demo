extern crate sfml;
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{Key, VideoMode, event, window_style};

extern crate num;
mod simulation;
use simulation::*;

mod rendering;
use rendering::*;


fn main() {

	let mut sim = Simulation::new(300, 300);
    sim.initialize();

    let mut window = RenderWindow::new(VideoMode::new_init(sim.width, sim.height, 32), "Simulation", window_style::CLOSE, &Default::default()).unwrap();
    window.set_vertical_sync_enabled(true);

    let mut viewport1 = Viewport::new(&sim, white_potential);
    let mut viewport2 = Viewport::new(&sim, red_blue_components_alpha_norm);

    loop {
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }


        window.clear(&Color::black());

        sim.update();

        viewport1.update();
        viewport2.update();

        window.draw(&viewport1.sprite);
        window.draw(&viewport2.sprite);

        window.display();
    }

}
