use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Result, Watcher};
use std::sync::mpsc;
use std::path::Path;

pub fn watch() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = recommended_watcher(tx)?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
    // Block forever, printing out events as they come in
    for res in rx {
        match res {
            Ok(event) => {
                match event.kind {
                    EventKind::Create(_) => {
                        if event.paths[0].exists() {
                            println!("CREATED...")
                        }
                    },
                    _ => println!("___"),
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
 
