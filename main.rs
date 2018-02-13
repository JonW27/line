mod display;
mod draw;

fn main() {
    let screen = display::new_screen(500, 500);
    let color = [0, 255, 0];

    draw::draw_line(0, 0, 20, 20, screen.clone(), color);
    draw::draw_line(5, 5, 20, 20, screen.clone(), color);
    display::save_ppm(screen.clone(), "coolio.ppm".to_string());
    // display(screen);
    // save_extension(screen, 'img.png');
}
