use std::collections::BTreeMap;

use anyhow::{anyhow, Result};

use crate::commands::CommandSpec;
use crate::config::ProfileConfig;

pub struct ExecutionContext<'a> {
    pub profile_name: &'a str,
    pub config_path: &'a str,
    pub modules: Vec<String>,
    pub commands: Vec<&'a str>,
}

pub enum ModuleResponse {
    Output(String),
    StartRepl(ReplState),
}

pub struct ReplState {
    pub banner: String,
    pub help: String,
}

pub trait Module: Send + Sync {
    fn module_type(&self) -> &'static str;
    fn command_specs(&self, module_name: &str) -> Vec<CommandSpec>;
    fn execute(&self, action: &str, ctx: &ExecutionContext<'_>) -> Result<ModuleResponse>;
}

#[derive(Default)]
pub struct ModuleRegistry {
    modules: BTreeMap<String, Box<dyn Module>>,
}

impl ModuleRegistry {
    pub fn from_profile(profile: &ProfileConfig) -> Result<Self> {
        let mut registry = Self::default();

        for (name, module_config) in &profile.module {
            let module = create_module(&module_config.module_type)?;
            registry.modules.insert(name.clone(), module);
        }

        Ok(registry)
    }

    pub fn execute_handler(&self, handler: &str, ctx: &ExecutionContext<'_>) -> Result<ModuleResponse> {
        let (module_name, action) = handler
            .split_once('.')
            .ok_or_else(|| anyhow!("invalid handler '{handler}', expected module.action"))?;

        let module = self
            .modules
            .get(module_name)
            .ok_or_else(|| anyhow!("module '{module_name}' is not configured"))?;

        module.execute(action, ctx)
    }

    pub fn describe(&self) -> Vec<String> {
        self.modules
            .iter()
            .map(|(name, module)| format!("{name}:{}", module.module_type()))
            .collect()
    }

    pub fn command_specs(&self) -> Vec<CommandSpec> {
        let mut commands = Vec::new();

        for (module_name, module) in &self.modules {
            commands.extend(module.command_specs(module_name));
        }

        commands
    }

    pub fn has_module(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }
}

fn create_module(module_type: &str) -> Result<Box<dyn Module>> {
    match module_type {
        "console" => Ok(Box::new(ConsoleModule)),
        "chatgpt_agent" => Ok(Box::new(ChatGptAgentModule)),
        "claude_agent" => Ok(Box::new(ClaudeAgentModule)),
        "mdns" => Ok(Box::new(MdnsModule)),
        "matter" => Ok(Box::new(MatterModule)),
        "bthome" => Ok(Box::new(BtHomeModule)),
        "shelly" => Ok(Box::new(ShellyModule)),
        other => Err(anyhow!("unknown module type '{other}'")),
    }
}

struct ConsoleModule;
struct ChatGptAgentModule;
struct ClaudeAgentModule;
struct MdnsModule;
struct MatterModule;
struct BtHomeModule;
struct ShellyModule;

impl Module for ConsoleModule {
    fn module_type(&self) -> &'static str {
        "console"
    }

    fn command_specs(&self, _module_name: &str) -> Vec<CommandSpec> {
        Vec::new()
    }

    fn execute(&self, action: &str, ctx: &ExecutionContext<'_>) -> Result<ModuleResponse> {
        match action {
            "repl" => Ok(ModuleResponse::StartRepl(ReplState {
                banner: format!(
                    "profile: {}\nconfig: {}\nmodules: {}\nType 'help' to see available actions.",
                    ctx.profile_name,
                    ctx.config_path,
                    ctx.modules.join(", ")
                ),
                help: format!(
                    "Built-in actions:\n  help          Show this help\n  commands      List profile commands\n  agent list    List available AI agent providers\n  agent chatgpt Scaffold a ChatGPT admin agent\n  agent claude  Scaffold a Claude admin agent\n  exit          Leave the REPL\n\nProfile commands:\n  {}",
                    if ctx.commands.is_empty() {
                        "<none>".to_string()
                    } else {
                        ctx.commands.join("\n  ")
                    }
                ),
            })),
            "help" => Ok(ModuleResponse::Output(
                "Console module provides the interactive REPL, command discovery, and AI agent provider setup scaffolding.".to_string(),
            )),
            other => Err(anyhow!("console does not support action '{other}'")),
        }
    }
}

impl Module for ChatGptAgentModule {
    fn module_type(&self) -> &'static str {
        "chatgpt_agent"
    }

    fn command_specs(&self, module_name: &str) -> Vec<CommandSpec> {
        vec![CommandSpec::new("agent-chatgpt", format!("{module_name}.setup"))]
    }

    fn execute(&self, action: &str, ctx: &ExecutionContext<'_>) -> Result<ModuleResponse> {
        match action {
            "setup" => Ok(ModuleResponse::Output(format!(
                "ChatGPT admin agent scaffold\nprovider = \"chatgpt\"\nname = \"{}-admin-chatgpt\"\nrole = \"admin\"\nentrypoint = \"repl\"\ncapabilities = [\"list-commands\", \"run-operations\", \"audit\"]",
                ctx.profile_name
            ))),
            other => Err(anyhow!("chatgpt_agent does not support action '{other}'")),
        }
    }
}

impl Module for ClaudeAgentModule {
    fn module_type(&self) -> &'static str {
        "claude_agent"
    }

    fn command_specs(&self, module_name: &str) -> Vec<CommandSpec> {
        vec![CommandSpec::new("agent-claude", format!("{module_name}.setup"))]
    }

    fn execute(&self, action: &str, ctx: &ExecutionContext<'_>) -> Result<ModuleResponse> {
        match action {
            "setup" => Ok(ModuleResponse::Output(format!(
                "Claude admin agent scaffold\nprovider = \"claude\"\nname = \"{}-admin-claude\"\nrole = \"admin\"\nentrypoint = \"repl\"\ncapabilities = [\"list-commands\", \"run-operations\", \"audit\"]",
                ctx.profile_name
            ))),
            other => Err(anyhow!("claude_agent does not support action '{other}'")),
        }
    }
}

impl Module for MdnsModule {
    fn module_type(&self) -> &'static str {
        "mdns"
    }

    fn command_specs(&self, module_name: &str) -> Vec<CommandSpec> {
        vec![CommandSpec::new("scan", format!("{module_name}.scan"))]
    }

    fn execute(&self, action: &str, _ctx: &ExecutionContext<'_>) -> Result<ModuleResponse> {
        match action {
            "scan" => Ok(ModuleResponse::Output("mDNS discovery scan started".to_string())),
            other => Err(anyhow!("mdns does not support action '{other}'")),
        }
    }
}

impl Module for MatterModule {
    fn module_type(&self) -> &'static str {
        "matter"
    }

    fn command_specs(&self, module_name: &str) -> Vec<CommandSpec> {
        vec![
            CommandSpec::new("devices", format!("{module_name}.list")),
            CommandSpec::new("commission", format!("{module_name}.commission")),
        ]
    }

    fn execute(&self, action: &str, _ctx: &ExecutionContext<'_>) -> Result<ModuleResponse> {
        match action {
            "list" => Ok(ModuleResponse::Output("Matter devices listed".to_string())),
            "commission" => Ok(ModuleResponse::Output("Matter commissioning flow started".to_string())),
            other => Err(anyhow!("matter does not support action '{other}'")),
        }
    }
}

impl Module for BtHomeModule {
    fn module_type(&self) -> &'static str {
        "bthome"
    }

    fn command_specs(&self, module_name: &str) -> Vec<CommandSpec> {
        vec![CommandSpec::new("scan", format!("{module_name}.scan"))]
    }

    fn execute(&self, action: &str, _ctx: &ExecutionContext<'_>) -> Result<ModuleResponse> {
        match action {
            "scan" => Ok(ModuleResponse::Output("BTHome scan started".to_string())),
            other => Err(anyhow!("bthome does not support action '{other}'")),
        }
    }
}

impl Module for ShellyModule {
    fn module_type(&self) -> &'static str {
        "shelly"
    }

    fn command_specs(&self, module_name: &str) -> Vec<CommandSpec> {
        vec![
            CommandSpec::new("devices", format!("{module_name}.list")),
            CommandSpec::new("toggle", format!("{module_name}.toggle")),
        ]
    }

    fn execute(&self, action: &str, _ctx: &ExecutionContext<'_>) -> Result<ModuleResponse> {
        match action {
            "list" => Ok(ModuleResponse::Output("Shelly devices listed".to_string())),
            "toggle" => Ok(ModuleResponse::Output("Shelly device toggled".to_string())),
            other => Err(anyhow!("shelly does not support action '{other}'")),
        }
    }
}
