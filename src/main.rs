
    use neutrino::{App, Window};

    fn main() {
        let mut window = Window::new();
        window.set_title("Hello World");
        window.set_size(320, 240);
    
        App::run(window);
    }

