#![feature(generic_const_exprs)]
use executable_storage::{Entry, ExecutableStorage};

struct Settings {
    resolution : Option<(usize, usize)>,
}

static mut SETTINGS: executable_storage::Entry<Settings, 8> = Entry {
    prefix: *b"SETTINGS",
    default_value: Settings {
        resolution: None,
    },
};

fn main() {
    let mut settings = ExecutableStorage::new(unsafe { &mut SETTINGS }).unwrap();
    match &settings.resolution {
        Some((width, height)) => {
            println!("Resolution is {width}x{height}.")
        },
        None => {
            println!("Resolution not set, fetching screen resolution.");
            // since i'm not gotta bother adding 500 libraries just to get the resolution of your monitor,
            // it's just gonna be set to these constants.
            const WIDTH : usize = 1920;
            const HEIGHT : usize = 1080; 
            settings.resolution = Some((WIDTH, HEIGHT));
            println!("Resolution is {WIDTH}x{HEIGHT}.");
        },
    }
}
