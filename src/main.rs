mod keystrokes;

fn main() {
    let cb = |_| println!("x");
    let mut listener = keystrokes::Listener {
        mouse_moved_callback: Some(&cb),
    };
    listener.listen();
}
