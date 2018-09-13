// std
use std::alloc::System;
use std::collections::HashMap;
use std::io;
use std::process::Command;

#[global_allocator]
static GLOBAL: System = System;

fn emoji_map<'a>() -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();

    map.insert("new", "✨");
    map.insert("feature", "✨");
    map.insert("bug", "🐛");
    map.insert("fix", "🐛");
    map.insert("security", "🔒");
    map.insert("performance", "📈");
    map.insert("improvement", "⚡");
    map.insert("breaking", "💥");
    map.insert("deprecated", "⚠️");
    map.insert("refactor", "👕");
    map.insert("docs", "✏️");
    map.insert("examples", "🍭");
    map.insert("add-test", "✅");
    map.insert("fix-test", "💚");
    map.insert("upgrade-dependencies", "⬆️");
    map.insert("downgrade-dependencies", "⬇️");
    map.insert("pin-dependencies", "📌");
    map.insert("config", "🔧");
    map.insert("build", "📦");
    map.insert("release-major", "🎊");
    map.insert("release-minor", "🎉");
    map.insert("release-patch", "✨");
    map.insert("release-deploy", "🚀");
    map.insert("revert", "🔙");
    map.insert("wip", "🚧");
    map.insert("add-files", "➕");
    map.insert("remove-files", "➖");
    map.insert("on", "🔛");

    map
}

fn category_map<'a>() -> HashMap<&'a str, &'a str> {
    let mut map = HashMap::new();

    map.insert("new", "new");
    map.insert("feature", "feature");
    map.insert("bug", "bug");
    map.insert("fix", "bug");
    map.insert("security", "security");
    map.insert("performance", "performance");
    map.insert("improvement", "improvement");
    map.insert("breaking", "breaking");
    map.insert("deprecated", "deprecated");
    map.insert("refactor", "refactor");
    map.insert("docs", "docs");
    map.insert("examples", "examples");
    map.insert("add-test", "test");
    map.insert("fix-test", "test");
    map.insert("upgrade-dependencies", "dependency");
    map.insert("downgrade-dependencies", "dependency");
    map.insert("pin-dependencies", "dependency");
    map.insert("config", "config");
    map.insert("build", "build");
    map.insert("release-major", "release");
    map.insert("release-minor", "release");
    map.insert("release-patch", "release");
    map.insert("release-deploy", "release");
    map.insert("revert", "revert");
    map.insert("wip", "wip");
    map.insert("add-files", "misc");
    map.insert("remove-files", "misc");
    map.insert("on", "misc");

    map
}

fn print_emojis() {
    let typs = emoji_map();

    println!("\nTypes");
    println!("=====\n");
    for (k, v) in typs.iter() {
        println!("{}\t{}", v, k);
    }
    println!();
}

fn main() {
    // Type
    let emoji_map = emoji_map();
    let category_map = category_map();
    let mut typ = String::new();
    let mut emoji = None;
    let mut category = None;
    while emoji.is_none() && category.is_none() {
        println!("Please enter the type of the change you're committing: ");
        io::stdin().read_line(&mut typ)
            .expect("Failed to read line");
        let typ = typ.trim();
        emoji = emoji_map.get(typ);
        if emoji.is_none() {
            print_emojis();
        }
        category = category_map.get(typ);
    }
    let emoji = emoji.unwrap();
    let category = category.unwrap();

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
    let message = if description.len() > 0 {
        if scope.len() > 0 {
            format!("\n{} {}({}): {}\n\n{}", emoji, category, scope, summary, description)
        } else {
            format!("\n{} {}: {}\n\n{}", emoji, category, summary, description)
        }
    } else {
        if scope.len() > 0 {
            format!("\n{} {}({}): {}", emoji, category, scope, summary)
        } else {
            format!("\n{} {}: {}", emoji, category, summary)
        }
    };

    Command::new("git")
        .args(&["commit", "-m", &message])
        .spawn()
        .expect("Failed to run git commit command");
}
