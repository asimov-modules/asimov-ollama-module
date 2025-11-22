# ASIMOV Ollama Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-ollama-module)](https://crates.io/crates/asimov-ollama-module)
[![Documentation](https://docs.rs/asimov-ollama-module/badge.svg)](https://docs.rs/asimov-ollama-module)

[ASIMOV] Ollama module.

## ✨ Features

- To be determined!

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code
- An available [Ollama](https://github.com/ollama/ollama) endpoint.

## ⬇️ Installation

### Installation with [ASIMOV CLI]

```bash
asimov module install ollama -v
```

### Installation from Source Code

```bash
cargo install asimov-ollama-module
```

## ⚙ Setup

This module uses an Ollama endpoint to generate responses.
On macOS a simple approach to run Ollama locally is:

```console
$ brew install ollama
$ brew services start ollama  # Start the service formula immediately and register it to launch at login (or boot).
```

Then you should be able to use `asimov-ollama-prompter`.
Alternatively downloading and running [the application](https://ollama.com/download) should also work.

### Downloading Models

You can download [a model](https://ollama.com/search) either through the CLI:

```console
$ ollama pull gemma3:1b
```

Or in the application:

![Model download in Ollama application](./etc/model_download.png)

## 👉 Examples

```console
$ echo "In two sentences, why is the sky blue?" | asimov-ollama-prompter -m gemma3:1b
The sky appears blue because of a phenomenon called Rayleigh scattering, where sunlight is split into different colors of light. Blue light is scattered more effectively by the tiny particles in the atmosphere than other colors, making it visible to our eyes.
```

## ⚙ Configuration

Provide a model name either by module configuration

```bash
asimov module config ollama
```

Or through environment variables

```bash
export ASIMOV_OLLAMA_MODEL="..."
```

### Optional configuration

| Name       | Environment Variable         | Default                  |
| ---------- | ---------------------------- | ------------------------ |
| `endpoint` | `ASIMOV_OLLAMA_API_ENDPOINT` | `http://localhost:11434` |

## 📚 Reference

### Prompt

```bash
echo "Why is the sky blue?" | asimov-ollama-prompter
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-ollama-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-ollama-module&text=asimov-ollama-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-ollama-module&title=asimov-ollama-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-ollama-module&t=asimov-ollama-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-ollama-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-ollama-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Rust]: https://rust-lang.org
