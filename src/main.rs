// std
use std::alloc::System;
use std::env;
use std::fs::{self, File};
use std::fmt;
use std::io::{self, Read, Write};
use std::process::{Command, Output};

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
    description: &'static str,

    // Some emojis seem to take "zero" space which causes some misalignment
    // when printing the list. This spacing is to make up for that.
    spacing: &'static str,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{} {:24} {}", self.emoji, self.spacing, self.typ, self.description)
    }
}

macro_rules! cat {
    ($category:expr, $typ:expr, $emoji:expr, $spacing:expr, $description:expr) => {
        Cat{category: $category, typ: $typ, emoji: $emoji, spacing: $spacing, description: $description}
    };
}

fn cats() -> Vec<Cat> {
    let mut cats = Vec::new();

    cats.push(cat!("new", "new", "⭐", "", "add **new feature**"));
    cats.push(cat!("feature", "feature", "⭐", "", "add **new feature**"));
    cats.push(cat!("bug", "bug", "🐛", "", "fix **bug** issue"));
    cats.push(cat!("security", "security", "🔒", "", "fix **security** issue"));
    cats.push(cat!("performance", "performance", "📈", "", "fix **performance** issue"));
    cats.push(cat!("improvement", "improvement", "⚡", "", "update **backwards-compatible** feature"));
    cats.push(cat!("breaking", "breaking", "💥", "", "update **backwards-incompatible** feature"));
    cats.push(cat!("deprecated", "deprecated", "⚠️", " ", "**deprecate** feature"));
    cats.push(cat!("update", "cosmetics", "💄", "", "update **UI/Cosmetic**"));
    cats.push(cat!("update", "other", "🆙", "", "update **other**"));
    cats.push(cat!("update", "i18n", "🌐", "", "update or fix **internationalization**"));
    cats.push(cat!("refactor", "refactor", "👕", "", "remove **linter**/strict/deprecation warnings or **refactoring** or code **layouting**"));
    cats.push(cat!("docs", "docs", "📝️", "", "update **documentation**"));
    cats.push(cat!("docs", "license", "©️️", " ", "decide or change **license**"));
    cats.push(cat!("examples", "examples", "🍭", "", "for **example** codes"));
    cats.push(cat!("test", "add-test", "✅", "", "add **tests**"));
    cats.push(cat!("test", "fix-test", "💚", "", "fix **tests** failure or **CI** building"));
    cats.push(cat!("dependency", "upgrade-dependencies", "⬆️", " ", "upgrade **dependencies**"));
    cats.push(cat!("dependency", "downgrade-dependencies", "⬇️", " ", "downgrade **dependencies**"));
    cats.push(cat!("dependency", "pin-dependencies", "📌", "", "pin **dependencies**"));
    cats.push(cat!("config", "config", "🔧", "", "update **configuration**"));
    cats.push(cat!("build", "build", "📦", "", "**packaging** or **bundling** or **building**"));
    cats.push(cat!("release", "release-initial", "🐣", "", "**initial** commit"));
    cats.push(cat!("release", "release-major", "🎊", "", "release **major** version"));
    cats.push(cat!("release", "release-minor", "🎉", "", "release **minor** version"));
    cats.push(cat!("release", "release-patch", "✨", "", "release **patch** version"));
    cats.push(cat!("release", "release-deploy", "🚀", "", "**deploy** to production enviroment"));
    cats.push(cat!("revert", "revert", "🔙", "", "**revert** commiting"));
    cats.push(cat!("wip", "wip", "🚧", "", "**WIP** commiting"));
    cats.push(cat!("resolve", "resolve", "🔀", "", "merge **conflict resolution**"));
    cats.push(cat!("add", "add", "➕", "", "**add** files, dependencies, ..."));
    cats.push(cat!("remove", "remove", "➖", "", "**remove** files, dependencies, ..."));
    cats.push(cat!("on", "on", "🔛", "", "**enable** feature and something ..."));

    cats
}

fn print_cats() {
    let cats = cats();

    println!("Types                       Description");
    println!("=======================================\n");
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
    final_args.push("-c");
    final_args.push("color.status=always");
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
        let output = Command::new("git")
            .args(&args)
            .output()
            .expect("Failed to run git config user.name command");
        print_command_output(output);

        let args = vec!["config", "user.email", &user.email];
        let output = Command::new("git")
            .args(&args)
            .output()
            .expect("Failed to run git config user.email command");
        print_command_output(output);
    }

    // Execute git commit command with our arguments
    let output = Command::new("git")
        .args(&final_args)
        .output()
        .expect("Failed to run git commit command");
    print_command_output(output);

    // Unset user section
    if user.is_some() {
        let args = vec!["config", "--remove-section", "user"];
        let output = Command::new("git")
            .args(&args)
            .output()
            .expect("Failed to run git config --remove-section user command");
        print_command_output(output);
    }
}

fn print_command_output(output: Output) {
    io::stdout().write(&output.stdout).unwrap();
    io::stderr().write(&output.stderr).unwrap();
}

fn main() {
    let matches =
        App::new("git-ez")
            .version("0.0.3")
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
