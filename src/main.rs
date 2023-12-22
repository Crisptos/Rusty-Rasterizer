use rusty_rasterizer::{global::{WIDTH, HEIGHT}, application::Application};

fn main() {
    
    let mut app: Application = Application::new(WIDTH, HEIGHT);

    app.init();

    while app.is_running(){
        app.update();
        app.draw_frame();
    }
    
}
