<div align="center">

# 🇪🇸 pizza-analysis-spanish

**Spanish text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--spanish-blue)](https://github.com/pizza-rs/analysis-spanish)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Spanish language analysis with light stemming and stop words.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `spanish_light_stem` | Spanish light stemmer |
| TokenFilter | `spanish_stop` | Spanish stop words (308 entries) |
| Analyzer | `spanish` | Full pipeline: lowercase → light_stem → stop |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_spanish::register_all(&mut factory);

let analyzer = factory.get_analyzer("spanish").unwrap();
// "bibliotecas" → "bibliotec"
```

## Installation

```toml
[dependencies]
pizza-analysis-spanish = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["spanish"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
