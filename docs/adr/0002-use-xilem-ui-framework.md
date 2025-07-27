# ADR 0002: Use Xilem as the UI Framework for Tuan

## Status

Accepted

## Context

Tuan is a new kind of IDE based on a graph-based structure rather than a traditional file hierarchy. Its user experience must be fluid, responsive, and highly interactive, with custom UI components, smooth transitions, and advanced visual representations.

To achieve this, we need fine-grained control over rendering, a modern graphics pipeline, and cross-platform support (including desktop, and possibly the web).

## Decision

We decided to use **Xilem** as the UI framework for Tuan.

Reasons:

- **Performance**: Xilem is built on top of `vello`, a GPU-accelerated 2D renderer, which enables smooth animations and fast rendering performance.
- **Cross-platform**: Xilem targets multiple platforms, including desktop and web, which aligns with Tuan’s multi-platform ambitions.
- **Canvas API**: Xilem exposes a powerful 2D Canvas API via Vello, making it easy to build highly customized visual components.
- **Full Rust stack**: The entire pipeline, from logic to rendering, stays in Rust, not requiring any JavaScript or other (slow) languages.

## Consequences

- **Pros**
  - GPU-accelerated rendering and smooth animations
  - Cross-platform support
  - Rust-native ecosystem
  - Powerful low-level Canvas API for drawing custom elements
- **Cons**
  - Still in active development (early-stage API may evolve)
  - Smaller ecosystem and community compared to `egui` or `iced`
  - Limited documentation and learning resources at the time of writing

## Alternatives Considered

- **egui**: That's easy to use and well-documented, but lacks the flexibility and fine control required for Tuan's custom UI components.
- **Floem**: A promising framework by the Lapce team, but lacks support for critical features such as opacity, limiting its suitability for rich animated interfaces.
- **Dioxus**: The cross-platform features is managed by a web view. It introduces startup artifacts like a white screen, which goes against our goal of a seamless, native-like UX.
- **Building a custom framework** (with Skia + winit): Would offer full control, but would require significant time and effort to develop and maintain.
