use std::{panic, thread, time::Duration};

fn main() {
    delay_main_panic();

    for _ in 1..250 {
        let srv = mockito::Server::new();
        std::mem::drop(srv);
    }

    println!("All good!");

    // maybe some fds will get closed? probably not.
    thread::sleep(Duration::from_secs(5));
}

// workaround for panic messages from different threads getting mixed together
fn delay_main_panic() {
    let def_panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if thread::current().name() == Some("main") {
            thread::sleep(Duration::from_secs(1));
        }
        def_panic_hook(info)
    }));
}
