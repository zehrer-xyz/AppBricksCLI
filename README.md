# AppBricksCLI

AppBricksCLI is a **generic, modular CLI runtime** that dynamically composes its behavior based on the **invocation name** and a corresponding **configuration profile**.

Instead of building many separate CLI tools, AppBricksCLI allows you to create a single binary that behaves differently depending on how it is called.

---

## ✨ Concept

AppBricksCLI follows a combination of:

* **Multi-call binary pattern** (like BusyBox)
* **Inversion of Control (IoC)**
* **Config-driven composition**
* **Modular plugin architecture**

The executable itself is just a **runtime host**.

The actual behavior is defined by:

1. The **name used to invoke the binary**
2. A matching **configuration file**
3. A set of **modules** dynamically loaded based on that configuration

---

## 🧠 How it works

### 1. Invocation

The same binary can be called under different names:

```bash
appbricks
shellyctl
chatgpt
claude
```

These are typically implemented via symlinks:

```bash
ln -s appbricks chatgpt
ln -s appbricks claude
```

---

### 2. Profile Resolution

The CLI determines how it was invoked:

```text
argv[0] → "chatgpt"
```

This maps to a profile:

```text
chatgpt → chatgpt.toml
```

---

### 3. Configuration

Each profile defines:

* which **modules** to load
* which module is the **head module**
* how modules are wired together

Example:

```toml
name = "appbricks"

[runtime]
head_module = "console"

[module.console]
type = "console"

[module.chatgpt_admin]
type = "chatgpt_agent"

[module.claude_admin]
type = "claude_agent"

[module.discovery]
type = "mdns"
```

In this model, modules register their own commands:

* `console` owns the default REPL experience
* `mdns` contributes `scan`
* `chatgpt_agent` contributes `agent-chatgpt`
* `claude_agent` contributes `agent-claude`

---

### 4. Runtime Composition

At startup:

1. Load profile config
2. Instantiate modules via registry
3. Ask each loaded module to register its commands
4. Start the head module when no explicit command is provided

The CLI behavior is fully defined by configuration.

---

## 🧱 Architecture

```
┌──────────────────────┐
│   AppBricksCLI       │
│   (runtime host)     │
└─────────┬────────────┘
          │
          ▼
┌──────────────────────┐
│ Profile Resolver     │
└─────────┬────────────┘
          ▼
┌──────────────────────┐
│ Config Loader        │
└─────────┬────────────┘
          ▼
┌──────────────────────┐
│ Module Registry      │
└─────────┬────────────┘
          ▼
┌──────────────────────┐
│ Module Instances     │
└─────────┬────────────┘
          ▼
┌──────────────────────┐
│ Command Registry     │
└─────────┬────────────┘
          ▼
┌──────────────────────┐
│ Runtime Execution    │
└──────────────────────┘
```

---

## 🔌 Modules

Modules are the building blocks of the system.

Each module:

* has a **type** (e.g. `console`, `mdns`, `chatgpt_agent`)
* is instantiated from config
* can:

  * expose commands
  * provide services
  * interact with other modules

One module can be designated as the **head module** for the profile. That module owns the default startup flow, such as a REPL or operator console.

Modules should be:

* loosely coupled
* interface-driven
* independently testable

---

## ⚙️ Design Principles

* **Single binary, multiple personalities**
* **Configuration over hardcoded logic**
* **Loose coupling via module interfaces**
* **Extensible via registry pattern**
* **Separation of runtime and behavior**

---

## 📦 Example Use Cases

* Network discovery tools
* Gateway controllers
* Automation CLIs with a built-in operator console
* DevOps utilities with multiple roles
* AI-assisted admin consoles with provider-specific entrypoints

---

## 🚀 Getting Started

### Build

```bash
cargo build --release
```

### Create a command via symlink

```bash
ln -s ./target/release/appbricks ./chatgpt
ln -s ./target/release/appbricks ./claude
```

### Run the default REPL

```bash
./appbricks
```

Inside the REPL:

```text
appbricks> help
appbricks> commands
appbricks> scan
appbricks> agent list
appbricks> agent chatgpt
appbricks> agent claude
appbricks> exit
```

Or call commands directly:

```bash
./appbricks scan
./appbricks agent-chatgpt
```

Provider-focused symlinks can expose narrower profiles:

```bash
./chatgpt
./chatgpt agent-chatgpt
./claude
./claude agent-claude
```

---

## 📁 Suggested Project Layout

```
appbricks-cli/
├── src/
│   ├── main.rs
│   ├── runtime/
│   ├── modules/
│   ├── commands/
│   └── config/
├── configs/
│   ├── default.toml
│   ├── chatgpt.toml
│   ├── claude.toml
│   └── shellyctl.toml
└── Cargo.toml
```

---

## 🔮 Future Ideas

* Dynamic module loading (plugins)
* Remote config profiles
* CLI schema validation
* Interactive TUI mode
* Home Assistant / Matter integration

---

## 🧩 Philosophy

AppBricksCLI is not "a CLI tool".

It is a **CLI construction kit**.

You don’t build tools —
you **compose them**.

---

## 📄 License

MIT (or your preferred license)

---

## 🤝 Contributing

Contributions are welcome!

Focus areas:

* module system
* config schema
* REPL experience
* AI agent provider integrations
* runtime orchestration
* developer experience

---
