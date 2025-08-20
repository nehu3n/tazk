#![allow(dead_code)]

use owo_colors::OwoColorize;

pub struct Logger;

impl Logger {
    pub fn info(msg: &str) {
        println!("{} {}", "ℹ".cyan().bold(), msg);
    }

    pub fn success(msg: &str) {
        println!("{} {}", "✔".green().bold(), msg.green());
    }

    pub fn warn(msg: &str) {
        println!("{} {}", "⚠".yellow().bold(), msg.yellow());
    }

    pub fn error(msg: &str) {
        eprintln!("{} {}", "✗".red().bold(), msg.red());
    }

    pub fn task_start(task_name: &str) {
        println!("{} running task: {}", "🐕".bold(), task_name.cyan().bold());
    }

    pub fn command(cmd: &str) {
        println!("   {} {}", "➜".blue().bold(), cmd.dimmed());
    }

    pub fn file_change(file: &str, pattern: &str) {
        println!(
            "{} change detected: {} (matched: {})",
            "📝".bold(),
            file.yellow().bold(),
            pattern.dimmed()
        );
    }

    pub fn watching_dir(dir: &str) {
        println!("{} watching directory: {}", "👀".bold(), dir.blue());
    }

    pub fn watching_patterns(patterns: &[String]) {
        println!(
            "{} watching files with patterns: {}",
            "👀".bold(),
            format!("{patterns:?}").blue()
        );
    }

    pub fn waiting() {
        println!(
            "{} waiting for file changes... {}",
            "⏳".bold(),
            "(Press Ctrl+C to exit)".dimmed()
        );
    }

    pub fn validation_ok() {
        println!("{} tasks file validation passed", "✅".bold());
    }

    pub fn validation_error(error: &str) {
        eprintln!("{} {}", "❌".bold(), error.red());
    }

    pub fn file_path(path: &str) {
        println!("{} file path: {}", "📁".bold(), path.blue().bold());
    }

    pub fn available_tasks() {
        println!("{} available tasks:", "📋".bold());
    }

    pub fn task_item(name: &str, desc: Option<&String>) {
        if let Some(description) = desc {
            println!("   {} {}: {}", "•".cyan().bold(), name.green().bold(), description.dimmed());
        } else {
            println!("   {} {}", "•".cyan().bold(), name.green().bold());
        }
    }

    pub fn dependency_propagated(task_name: &str) {
        println!("{} propagating to dependent task: {}", "🔄".bold(), task_name.cyan().bold());
    }

    pub fn debug(msg: &str) {
        if cfg!(debug_assertions) {
            println!("{} {}", "🔍".dimmed(), msg.dimmed());
        }
    }

    pub fn banner() {
        println!();
        println!(
            "{} {} {}",
            "🐕".bold(),
            "Tazk".cyan().bold(),
            "- Lightweight, agnostic, fast and easy task runner.".dimmed()
        );
        println!();
    }

    pub fn separator() {
        println!("{}", "─".repeat(50).dimmed());
    }
}
