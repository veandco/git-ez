// std
use std::alloc::System;
use std::env;
use std::fs::{self, File};
use std::fmt;
use std::io::{self, Read};
use std::process::Command;

// clap
extern crate clap;
use clap::{Arg, App, SubCommand};

// dirs
extern crate dirs;

// serde
#[macro_use]
extern crate serde_derive;

// toml
extern crate toml;

#[global_allocator]
static GLOBAL: System = System;

#[derive(Deserialize, Serialize, Clone)]
struct Config {
    users: Vec<User>,
}

impl Config {
    fn new() -> Config {
        Config {
            users: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct User {
    name: String,
    email: String,
    ip_addresses: Vec<String>
}

struct Cat {
    category: &'static str,
    typ: &'static str,
    emoji: &'static str,

    // Some emojis seem to take "zero" space which causes some misalignment
    // when printing the list. This spacing is to make up for that.
    spacing: &'static str,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{} {}", self.emoji, self.spacing, self.typ)
    }
}

macro_rules! cat {
    ($category:expr, $typ:expr, $emoji:expr, $spacing:expr) => {
        Cat{category: $category, typ: $typ, emoji: $emoji, spacing: $spacing}
    };
}

fn cats() -> Vec<Cat> {
    let mut cats = Vec::new();

    cats.push(cat!("new", "new", "⭐", ""));
    cats.push(cat!("feature", "feature", "⭐", ""));
    cats.push(cat!("bug", "bug", "🐛", ""));
    cats.push(cat!("bug", "fix", "🐛", ""));
    cats.push(cat!("security", "security", "🔒", ""));
    cats.push(cat!("performance", "performance", "📈", ""));
    cats.push(cat!("improvement", "improvement", "⚡", ""));
    cats.push(cat!("breaking", "breaking", "💥", ""));
    cats.push(cat!("deprecated", "deprecated", "⚠️", " "));
    cats.push(cat!("update", "cosmetics", "💄", ""));
    cats.push(cat!("update", "other", "🆙", ""));
    cats.push(cat!("update", "i18n", "🌐", ""));
    cats.push(cat!("refactor", "refactor", "👕", ""));
    cats.push(cat!("docs", "docs", "📝️", " "));
    cats.push(cat!("docs", "license", "©️️", " "));
    cats.push(cat!("examples", "examples", "🍭", ""));
    cats.push(cat!("test", "add-test", "✅", ""));
    cats.push(cat!("test", "fix-test", "💚", ""));
    cats.push(cat!("dependency", "upgrade-dependencies", "⬆️", " "));
    cats.push(cat!("dependency", "downgrade-dependencies", "⬇️", " "));
    cats.push(cat!("dependency", "pin-dependencies", "📌", ""));
    cats.push(cat!("config", "config", "🔧", ""));
    cats.push(cat!("build", "build", "📦", ""));
    cats.push(cat!("release", "release-initial", "🐣", ""));
    cats.push(cat!("release", "release-major", "🎊", ""));
    cats.push(cat!("release", "release-minor", "🎉", ""));
    cats.push(cat!("release", "release-patch", "✨", ""));
    cats.push(cat!("release", "release-deploy", "🚀", ""));
    cats.push(cat!("revert", "revert", "🔙", ""));
    cats.push(cat!("wip", "wip", "🚧", ""));
    cats.push(cat!("resolve", "resolve", "🔀", ""));
    cats.push(cat!("add", "add", "➕", ""));
    cats.push(cat!("remove", "remove", "➖", ""));
    cats.push(cat!("on", "on", "🔛", ""));

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

fn config() -> Option<Config> {
    if let Some(home) = dirs::home_dir() {
        let config_path = format!("{}/{}", home.display(), ".gitez");
        if let Ok(mut file) = File::open(&config_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(config) = toml::de::from_str(&contents) {
                    return Some(config);
                }
            }
        }
    }
    None
}

fn current_ip_address() -> String {
    if let Ok(ssh_client) = env::var("SSH_CLIENT") {
        let fields: Vec<_> = ssh_client.split(' ').collect();
        if !fields.is_empty() {
            return fields[0].to_string();
        }
    }
    String::new()
}

fn save_config(config: &Config) {
    if let Some(home) = dirs::home_dir() {
        let config_path = format!("{}/{}", home.display(), ".gitez");
        fs::write(&config_path, toml::to_string(&config).unwrap())
            .expect("Failed to save config file");
    }
}

fn current_user_from_config(config: &mut Config) -> Option<&mut User> {
    let mut found = None;
    for i in 0..config.users.len() {
        for ip_address in &mut config.users[i].ip_addresses {
            if ip_address == &current_ip_address() {
                found = Some(i);
                break;
            }
        }
    }
    if let Some(i) = found {
        return Some(&mut config.users[i]);
    }
    None
}

fn matching_user_from_config<'a>(name: &str, email: &str, config: &'a mut Config) -> Option<&'a mut User> {
    let mut found = None;
    for i in 0..config.users.len() {
        if config.users[i].name == name && config.users[i].email == email {
            found = Some(i);
            break;
        }
    }
    if let Some(i) = found {
        return Some(&mut config.users[i]);
    }
    None
}

fn add_user(name: &str, email: &str) {
    let current_ip_address = current_ip_address();
    if current_ip_address.is_empty() {
        return;
    }

    let config = config();
    if let Some(mut config) = config {
        let mut done = false;
        {
            let mut user = matching_user_from_config(name, email, &mut config);
            if let Some(mut user) = user {
                if user.ip_addresses.contains(&current_ip_address) {
                    return;
                }
                user.ip_addresses.push(current_ip_address.clone());
                done = true;
            }
        }
        if !done {
            config.users.push(User{
                name: name.to_string(),
                email: email.to_string(),
                ip_addresses: vec![current_ip_address],
            });
        }
        save_config(&config);
    } else {
        let mut config = Config::new();
        config.users.push(User{
            name: name.to_string(),
            email: email.to_string(),
            ip_addresses: vec![current_ip_address],
        });
        save_config(&config);
    }
}

fn remove_user(name: Option<&str>, email: Option<&str>) {
    let config = config();
    if let Some(mut config) = config {
        if let Some(name) = name {
            if let Some(email) = email {
                config.users = config.users.iter().filter(|user| !(user.name == name && user.email == email)).cloned().collect();
                println!("Removed user(s) with name = \"{}\" and email = \"{}\"", name, email);
            } else {
                config.users = config.users.iter().filter(|user| user.name != name).cloned().collect();
                println!("Removed user(s) with name = \"{}\"", name);
            }
        } else if let Some(email) = email {
            config.users = config.users.iter().filter(|user| user.email != email).cloned().collect();
            println!("Removed user(s) with email = \"{}\"", email);
        } else {
            let current_ip_address = current_ip_address();
            config.users = config.users.iter().filter(|user| !user.ip_addresses.contains(&current_ip_address)).cloned().collect();
            println!("Removed user(s) with IP address = \"{}\"", &current_ip_address);
        }

        save_config(&config);
    }
}

fn clear_users() {
    let config = config();
    if let Some(mut config) = config {
        config.users.clear();
        save_config(&config);
    }
}

fn list_users() {
    let config = config();
    if let Some(config) = config {
        config.users.iter().for_each(|user| {
            println!("{} <{}>", user.name, user.email);
        });
    } else {
        println!("No previous configuration found")
    }
}

fn commit<'a>(user: &Option<&'a mut User>, git_options: &[&'a str]) {
    let cats = cats();
    let mut cat = None;

    // Type
    let mut typ = String::new();
    while cat.is_none() {
        // Get type of change from user
        typ.clear();
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
            format!("\n{}{} {}: {}", cat.emoji, cat.spacing, cat.category, summary)
        } else {
            format!("\n{}{} {}({}): {}", cat.emoji, cat.spacing, cat.category, scope, summary)
        }
    } else if scope.is_empty() {
        format!("\n{}{} {}: {}\n\n{}", cat.emoji, cat.spacing, cat.category, summary, description)
    } else {
        format!("\n{}{} {}({}): {}\n\n{}", cat.emoji, cat.spacing, cat.category, scope, summary, description)
    };

    // Add additional Git command-line arguments that were set by user
    let mut final_args = Vec::new();
    final_args.push("commit");
    if !git_options.is_empty() {
        for arg in &git_options[0..] {
            final_args.push(&arg);
        }
    }

    // Put the commit message
    final_args.push("-m");
    final_args.push(&message);

    // Set user name and email
    if let Some(ref user) = user {
        let args = vec!["config", "user.name", &user.name];
        Command::new("git")
            .args(&args)
            .output()
            .expect("Failed to run git config user.name command");
        let args = vec!["config", "user.email", &user.email];
        Command::new("git")
            .args(&args)
            .output()
            .expect("Failed to run git config user.email command");
    }

    // Execute git commit command with our arguments
    Command::new("git")
        .args(&final_args)
        .output()
        .expect("Failed to run git commit command");

    // Unset user section
    if user.is_some() {
        let args = vec!["config", "--remove-section", "user"];
        Command::new("git")
            .args(&args)
            .output()
            .expect("Failed to run git config --remove-section user command");
    }
}

fn main() {
    let matches =
        App::new("git-ez")
            .version("0.0.1")
            .about("Git commit helper command that includes emoji!")
            .arg(Arg::with_name("git")
                .long("git")
                .short("g")
                .takes_value(true)
                .help("additional git options"))
            .subcommand(SubCommand::with_name("user")
                .about("User-related subcommand")
                .subcommand(SubCommand::with_name("add")
                    .about("Add a user")
                    .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .required(true)
                        .takes_value(true)
                        .help("name of the user"))
                    .arg(Arg::with_name("email")
                        .long("email")
                        .short("e")
                        .required(true)
                        .takes_value(true)
                        .help("email of the user")))
                .subcommand(SubCommand::with_name("remove")
                    .about("Remove the current user")
                    .arg(Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .required(false)
                        .takes_value(true)
                        .help("name of the user"))
                    .arg(Arg::with_name("email")
                        .long("email")
                        .short("e")
                        .required(false)
                        .takes_value(true)
                        .help("email of the user")))
                .subcommand(SubCommand::with_name("clear")
                    .about("Clear all users"))
                .subcommand(SubCommand::with_name("list")
                    .about("List all users")))
            .get_matches();

    if let Some(matches) = matches.subcommand_matches("user") {
        if let Some(matches) = matches.subcommand_matches("add") {
            let name = matches.value_of("name").unwrap();
            let email = matches.value_of("email").unwrap();
            return add_user(name, email);
        } else if let Some(matches) = matches.subcommand_matches("remove") {
            return remove_user(matches.value_of("name"), matches.value_of("email"));
        } else if matches.subcommand_matches("clear").is_some() {
            return clear_users();
        } else if matches.subcommand_matches("list").is_some() {
            return list_users();
        }
    }

    // Check if we already have git-ez config file
    let config = config();
    let git_options =
        match matches.values_of("git") {
            Some(values) => values.collect::<Vec<_>>(),
            None => Vec::new(),
        };
    if let Some(mut config) = config {
        commit(&current_user_from_config(&mut config), &git_options);
    } else {
        commit(&None, &git_options);
    }
}

#[test]
fn test_print_cats() {
    print_cats();
}

#[test]
fn test_current_ip_address() {
    println!("Current IP Address: {}", current_ip_address());
}