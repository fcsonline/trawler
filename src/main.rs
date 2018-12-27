use notify::{DebouncedEvent, Watcher, watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

mod lib;
mod steps;

use self::lib::pipeline::Pipeline;
use self::lib::buffer::Buffer;

fn pipeline(path: PathBuf) {
    let mut file = File::open(path.to_str().unwrap()).expect("Unable to open");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Unable to read");

    let meta = HashMap::new();

    let buffer = Buffer {
        content,
        meta
    };

    println!("Content: {:?}", buffer);

    let mut pipelines = HashMap::new();
    let pipeline = Pipeline {
        steps: vec![
            "echo".to_string(),
            "reverse".to_string(),
            "write".to_string()
        ]
    };

    pipelines.insert("svg".to_string(), pipeline);

    let pipe = pipelines.get("svg").unwrap();

    println!("Pipeline: {:?}", pipe);

    pipe.execute(buffer);

}

fn handle(event: DebouncedEvent) {
    println!("Processing: {:?}", event);
    // let path = Path::new(event.path.unwrap());
    match event {
        DebouncedEvent::Create(event) => pipeline(event),
        _ => println!("FCS2")
    }
}

fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher.watch("./", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => handle(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
