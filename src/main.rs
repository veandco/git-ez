// std
use std::alloc::System;
use std::env;
use std::fmt;
use std::io;
use std::process::Command;

#[global_allocator]
static GLOBAL: System = System;

struct Cat {
    category: &'static str,
    typ: &'static str,
    emoji: &'static str,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:32}{}", self.typ, self.emoji)
    }
}

macro_rules! cat {
    ($category:expr, $typ:expr, $emoji:expr) => {
        Cat{category: $category, typ: $typ, emoji: $emoji}
    };
}

fn cats() -> Vec<Cat> {
    let mut cats = Vec::new();

    cats.push(cat!("new", "new", "✨"));
    cats.push(cat!("feature", "feature", "✨"));
    cats.push(cat!("bug", "bug", "🐛"));
    cats.push(cat!("bug", "fix", "🐛"));
    cats.push(cat!("security", "security", "🔒"));
    cats.push(cat!("performance", "performance", "📈"));
    cats.push(cat!("improvement", "improvement", "⚡"));
    cats.push(cat!("breaking", "breaking", "💥"));
    cats.push(cat!("deprecated", "deprecated", "⚠️"));
    cats.push(cat!("update", "cosmetics", "💄"));
    cats.push(cat!("update", "other", "🆙"));
    cats.push(cat!("update", "i18n", "🌐"));
    cats.push(cat!("refactor", "refactor", "👕"));
    cats.push(cat!("docs", "docs", "✏️"));
    cats.push(cat!("docs", "license", "©️️"));
    cats.push(cat!("examples", "examples", "🍭"));
    cats.push(cat!("test", "add-test", "✅"));
    cats.push(cat!("test", "fix-test", "💚"));
    cats.push(cat!("dependency", "upgrade-dependencies", "⬆️"));
    cats.push(cat!("dependency", "downgrade-dependencies", "⬇️"));
    cats.push(cat!("dependency", "pin-dependencies", "📌"));
    cats.push(cat!("config", "config", "🔧"));
    cats.push(cat!("build", "build", "📦"));
    cats.push(cat!("release", "release-initial", "🐣"));
    cats.push(cat!("release", "release-major", "🎊"));
    cats.push(cat!("release", "release-minor", "🎉"));
    cats.push(cat!("release", "release-patch", "✨"));
    cats.push(cat!("release", "release-deploy", "🚀"));
    cats.push(cat!("revert", "revert", "🔙"));
    cats.push(cat!("wip", "wip", "🚧"));
    cats.push(cat!("resolve", "resolve", "🔀"));
    cats.push(cat!("add", "add", "➕"));
    cats.push(cat!("remove", "remove", "➖"));
    cats.push(cat!("on", "on", "🔛"));

    cats
}

fn print_cats() {
    let cats = cats();

    println!("\nTypes");
    println!("=====\n");
    for cat in &cats {
        println!("{}", cat);
    }
    println!();
}

fn main() {
    let cats = cats();
    let mut cat = None;

    // Type
    let mut typ = String::new();
    while cat.is_none() {
        // Get type of change from user
        println!("Please enter the type of the change you're committing: ");
        io::stdin().read_line(&mut typ)
            .expect("Failed to read line");
        let typ = typ.trim();

        // Find cat
        cat = cats.iter().find(|cat| cat.typ == typ);
        if cat.is_none() {
            print_cats();
        }
    }

    // Scope
    println!("\nScope: ");
    let mut scope = String::new();
    io::stdin().read_line(&mut scope)
        .expect("Failed to read scope :(");
    let scope = scope.trim();

    // Summary
    println!("\nSummary: ");
    let mut summary = String::new();
    io::stdin().read_line(&mut summary)
        .expect("Failed to read summary :(");
    let summary = summary.trim();

    // Description
    println!("\nDescription:");
    let mut description = String::new();
    io::stdin().read_line(&mut description)
        .expect("Failed to read description :(");
    let description = description.trim();

    // Commit message
    let cat = cat.unwrap();
    let message = if description.is_empty() {
        if scope.is_empty() {
            format!("\n{} {}: {}", cat.emoji, cat.category, summary)
        } else {
            format!("\n{} {}({}): {}", cat.emoji, cat.category, scope, summary)
        }
    } else if scope.is_empty() {
        format!("\n{} {}: {}\n\n{}", cat.emoji, cat.category, summary, description)
    } else {
        format!("\n{} {}({}): {}\n\n{}", cat.emoji, cat.category, scope, summary, description)
    };

    // Add additional command-line arguments that were set by user
    let os_args = env::args();
    let additional_args: Vec<String> = os_args.map(|arg| arg.to_string().clone()).collect();
    let mut final_args = Vec::new();
    final_args.push("commit");
    for arg in &additional_args[1..] {
        final_args.push(&arg);
    }

    // Put the commit message
    final_args.push("-m");
    final_args.push(&message);

    Command::new("git")
        .args(&final_args)
        .spawn()
        .expect("Failed to run git commit command");
}