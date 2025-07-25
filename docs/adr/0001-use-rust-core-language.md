# ADR 0001: Use Rust as the Core Language for Tuan IDE Development

## Status

Accepted

## Context

Tuan is a whole new IDE concept with a non-hierarchical, graph-based data model. Unlike VS Code, Tuan aims to offer a UX that feels truly native: smooth animations, real-time feedback, and minimal latency.

## Decision

We decided to use **Rust** as the primary programming language for Tuan.

Reasons:

- **Performance**: Rust is reputable for its speed and efficiency, making it suitable for building a high-performance IDE.
- **Similar projects**: There are successful IDEs and tools (like [Zed](https://zed.dev/), [Lapce](https://lap.dev/lapce/) or [Helix](https://helix-editor.com/)) built in Rust, proving its viability for this domain. It also allows us to draw inspiration from their architecture, design patterns, and libraries.
- **Personal motivation**: Rust is the language I (Arthur) want to learn for several years. This project is a great opportunity to dive deep into it.
- **Community and Ecosystem**: Rust has a growing ecosystem with a strong community, providing libraries and tools that can accelerate development. Moreover, the documentations and resources available for Rust are excellent, which is crucial for a team that is new to the language.
- **WebAssembly**: Rust has excellent support for WebAssembly, which could be beneficial if we decide to build a web-based version of Tuan in the future.

## Consequences

- **Pros**
  - High performance and low-level control
  - Strong type system and memory safety
  - Growing ecosystem
  - Active community and excellent documentation
  - Opportunity to learn and grow in a modern language
  - WASM support
- **Cons**
  - Steeper learning curve for those unfamiliar with Rust
  - Smaller ecosystem compared to more established languages like JavaScript

## Alternatives Considered

- **JavaScript/TypeScript**: While it has been used for many IDEs (like [VS Code](https://code.visualstudio.com/), [Cursor](https://cursor.so/), etc.), its runtime limitations (e.g., GC pauses, single-threaded execution) is also the reason those IDEs feel sluggish and unresponsive at times.
