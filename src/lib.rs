use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use log::info;
use domain::GestureDetectorService;

#[no_mangle]
fn android_main(_: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Info));

    let mut quit = false;
    let mut redraw_pending = true;
    let mut render_state: Option<()> = Default::default();
    let mut service = GestureDetectorService::new();

    while !quit {
        // Todo, loop forever on a background thread
        let handle = thread::spawn(|| {
            loop {
                // Do use the service to detect gestures
                service.check();
                thread::sleep(Duration::from_secs(1));
            }
        });
    
        // Main thread can do other work here
        // In this case, we'll just wait for a bit to let the background thread run
        for _ in 0..5 {
            println!("Main thread is working...");
            thread::sleep(Duration::from_secs(2));
        }
    }
}

