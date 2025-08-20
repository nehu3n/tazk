use glob::Pattern;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

pub fn watch_task<F: Fn() + Send + Sync + 'static>(
    patterns: &[String],
    debounce_ms: u64,
    callback: F,
) {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, Config::default().with_poll_interval(Duration::from_millis(100)))
            .expect("failed to create file watcher");

    let compiled_patterns: Vec<Pattern> =
        patterns.iter().map(|p| Pattern::new(p).expect("invalid glob pattern")).collect();

    let mut watched_dirs = HashSet::new();

    for pattern in patterns {
        let path = Path::new(pattern);

        if let Some(parent) = path.parent() {
            let dir_to_watch = if parent.as_os_str().is_empty() { Path::new(".") } else { parent };
            watched_dirs.insert(dir_to_watch.to_path_buf());
        } else {
            watched_dirs.insert(Path::new(".").to_path_buf());
        }
    }

    for dir in &watched_dirs {
        println!("watching directory: {}", dir.display());
        watcher
            .watch(dir, RecursiveMode::Recursive)
            .unwrap_or_else(|_| panic!("failed to watch path: {}", dir.display()));
    }

    println!("watching files for changes with patterns: {patterns:?}");

    let mut last_event_time = Instant::now();
    let debounce_duration = Duration::from_millis(debounce_ms);

    loop {
        match rx.recv() {
            Ok(event_result) => match event_result {
                Ok(event) => {
                    let mut should_trigger = false;

                    for path in &event.paths {
                        let is_relevant_event = matches!(
                            &event.kind,
                            notify::EventKind::Modify(_)
                                | notify::EventKind::Create(_)
                                | notify::EventKind::Remove(_)
                        );

                        if !is_relevant_event {
                            continue;
                        }

                        let relative_path = if path.is_absolute() {
                            if let Ok(current_dir) = env::current_dir() {
                                path.strip_prefix(current_dir).unwrap_or(path).to_path_buf()
                            } else {
                                path.to_path_buf()
                            }
                        } else {
                            path.to_path_buf()
                        };

                        for pattern in &compiled_patterns {
                            if pattern.matches_path(&relative_path) {
                                println!(
                                    "change detected: {} (matched pattern: {})",
                                    relative_path.display(),
                                    pattern.as_str()
                                );
                                should_trigger = true;
                                break;
                            }
                        }

                        if should_trigger {
                            break;
                        }
                    }

                    if should_trigger {
                        let now = Instant::now();
                        if now.duration_since(last_event_time) >= debounce_duration {
                            last_event_time = now;
                            callback();
                        }
                    }
                }
                Err(e) => eprintln!("watch error: {e:?}"),
            },
            Err(e) => {
                eprintln!("channel error: {e:?}");
                break;
            }
        }
    }
}
