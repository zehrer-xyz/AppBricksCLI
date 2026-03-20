use std::env;
use std::io::{self, Write};
use std::path::Path;

use anyhow::{anyhow, Result};

use crate::commands::CommandRegistry;
use crate::config::load_profile;
use crate::modules::{ExecutionContext, ModuleRegistry, ModuleResponse};

pub fn run() -> Result<()> {
    let invocation = env::args()
        .next()
        .and_then(|arg0| {
            Path::new(&arg0)
                .file_name()
                .map(|value| value.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "appbricks".to_string());

    let command = env::args().nth(1);
    let (profile_path, profile) = load_profile(&invocation)?;
    let modules = ModuleRegistry::from_profile(&profile)?;

    let mut commands = CommandRegistry::new();
    for spec in modules.command_specs() {
        commands.register(spec.name, spec.handler);
    }
    for (name, cfg) in &profile.commands {
        commands.register(name.clone(), cfg.handler.clone());
    }

    match command {
        Some(command_name) => {
            dispatch_command(
                &command_name,
                &profile.name,
                &profile_path.display().to_string(),
                &modules,
                &commands,
            )?;
        }
        None => {
            if let Some(handler) = resolve_default_handler(&profile, &modules)? {
                handle_response(
                    modules.execute_handler(
                        &handler,
                        &build_context(&profile.name, &profile_path.display().to_string(), &modules, &commands),
                    )?,
                    &profile.name,
                    &profile_path.display().to_string(),
                    &modules,
                    &commands,
                )?;
            } else {
                println!("profile: {}", profile.name);
                println!("config: {}", profile_path.display());
                println!("modules: {}", modules.describe().join(", "));
                println!("commands: {}", commands.names().join(", "));
                println!("usage: {invocation} <command>");
            }
        }
    }

    Ok(())
}

fn resolve_default_handler(
    profile: &crate::config::ProfileConfig,
    modules: &ModuleRegistry,
) -> Result<Option<String>> {
    if let Some(head_module) = profile.runtime.head_module.as_deref() {
        if !modules.has_module(head_module) {
            return Err(anyhow!("configured head_module '{head_module}' is not loaded"));
        }

        return Ok(Some(format!("{head_module}.repl")));
    }

    Ok(profile.runtime.default_handler.clone())
}

fn dispatch_command(
    command_name: &str,
    profile_name: &str,
    config_path: &str,
    modules: &ModuleRegistry,
    commands: &CommandRegistry,
) -> Result<()> {
    let handler = commands
        .resolve(command_name)
        .ok_or_else(|| anyhow!("unknown command '{command_name}' for profile '{profile_name}'"))?;
    let ctx = build_context(profile_name, config_path, modules, commands);
    handle_response(modules.execute_handler(handler, &ctx)?, profile_name, config_path, modules, commands)
}

fn handle_response(
    response: ModuleResponse,
    profile_name: &str,
    config_path: &str,
    modules: &ModuleRegistry,
    commands: &CommandRegistry,
) -> Result<()> {
    match response {
        ModuleResponse::Output(output) => {
            println!("{output}");
            Ok(())
        }
        ModuleResponse::StartRepl(repl) => start_repl(repl.banner, repl.help, profile_name, config_path, modules, commands),
    }
}

fn start_repl(
    banner: String,
    help_text: String,
    profile_name: &str,
    config_path: &str,
    modules: &ModuleRegistry,
    commands: &CommandRegistry,
) -> Result<()> {
    println!("{banner}");

    let stdin = io::stdin();
    loop {
        print!("{profile_name}> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            println!();
            break;
        }

        let input = line.trim();
        match input {
            "" => continue,
            "exit" | "quit" => break,
            "help" => println!("{help_text}"),
            "commands" => {
                if commands.is_empty() {
                    println!("No profile commands registered.");
                } else {
                    for name in commands.names() {
                        println!("{name}");
                    }
                }
            }
            "agent" | "agent list" => {
                println!("Available AI agent providers:");
                if commands.resolve("agent-chatgpt").is_some() {
                    println!("chatgpt");
                }
                if commands.resolve("agent-claude").is_some() {
                    println!("claude");
                }
            }
            "agent chatgpt" => {
                if let Err(err) = dispatch_command("agent-chatgpt", profile_name, config_path, modules, commands) {
                    println!("error: {err}");
                }
            }
            "agent claude" => {
                if let Err(err) = dispatch_command("agent-claude", profile_name, config_path, modules, commands) {
                    println!("error: {err}");
                }
            }
            other => {
                if let Err(err) = dispatch_command(other, profile_name, config_path, modules, commands) {
                    println!("error: {err}");
                }
            }
        }
    }

    Ok(())
}

fn build_context<'a>(
    profile_name: &'a str,
    config_path: &'a str,
    modules: &'a ModuleRegistry,
    commands: &'a CommandRegistry,
) -> ExecutionContext<'a> {
    ExecutionContext {
        profile_name,
        config_path,
        modules: modules.describe(),
        commands: commands.names(),
    }
}
