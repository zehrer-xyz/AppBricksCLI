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
matterctl
bthomectl
shellyctl
dorf
```

These are typically implemented via symlinks:

```bash
ln -s appbricks matterctl
ln -s appbricks bthomectl
```

---

### 2. Profile Resolution

The CLI determines how it was invoked:

```text
argv[0] → "matterctl"
```

This maps to a profile:

```text
matterctl → matterctl.toml
```

---

### 3. Configuration

Each profile defines:

* which **modules** to load
* which **commands** are available
* how modules are wired together

Example:

```toml
name = "matterctl"

[module.discovery]
type = "mdns"

[module.matter]
type = "matter"

[commands.scan]
handler = "discovery.scan"

[commands.devices]
handler = "matter.list"
```

---

### 4. Runtime Composition

At startup:

1. Load profile config
2. Instantiate modules via registry
3. Register commands
4. Start runtime

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

* has a **type** (e.g. `mdns`, `matter`, `bthome`)
* is instantiated from config
* can:

  * expose commands
  * provide services
  * interact with other modules

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

* IoT tooling (Matter, Shelly, BTHome)
* Network discovery tools
* Gateway controllers
* Automation CLIs
* DevOps utilities with multiple roles

---

## 🚀 Getting Started

### Build

```bash
cargo build --release
```

### Create a command via symlink

```bash
ln -s ./target/release/appbricks ./matterctl
```

### Run

```bash
./matterctl scan
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
│   ├── matterctl.toml
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
* runtime orchestration
* developer experience

---
