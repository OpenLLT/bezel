# Architecture

bezel is a gpui component library with SwiftUI's architecture and shadcn's
spirit: a small set of layered crates you build real apps on, extracted from
working application code — never invented ahead of need.

## Layout

```
crates/theme     tokens + appearance      (the @Environment layer)
crates/motion    animation vocabulary     (the Animation/transition layer)
crates/ui        components               (the View layer)
apps/gallery     the dev surface — every component rendered in a real window
```

Library crates live in `crates/`, binaries in `apps/`. Each crate depends
only downward: `ui → motion + theme`, `motion → (theme in tests)`, `theme →
gpui alone`.

## Laws

1. **Style flows through the environment.** Components read `Theme::of(cx)`
   (a gpui `Global`) at paint time — SwiftUI's `@Environment`. No color,
   font, or size parameters on component functions.
2. **SwiftUI vocabulary.** Widgets are named for their SwiftUI analog:
   `toggle`, `divider`, `group_box`, `material`, `button_prominent`,
   `redacted_rows`. Components are plain functions returning gpui elements —
   no component structs, no builder knobs, no style traits. Customization is
   editing the source.
3. **Motion is named.** Every animation comes from the `MotionSpec` catalog
   in `bezel-motion`; pure phase math lives in `motion::phase` and is
   unit-tested. No inline durations or curves in components.
4. **Numbers drive layout, colors are paint.** Layout constants are plain
   numbers on `Theme`; no layout ever depends on which color is painted.

## Dependencies

gpui is pinned by rev to [crabtalk/zed](https://github.com/crabtalk/zed),
our fork of gpui's home repo (the crates.io release trails the API by
months). gpui patches we need (first up: a backdrop-blur primitive for
`ui::material`) land on that fork. For in-workspace gpui development,
`.cargo/config.toml` (gitignored) patches gpui to a sibling `../zed`
checkout.

## Roadmap

Next extractions from comet, in order of value: markdown renderer
(streaming, block-incremental), tree-sitter syntax crate + highlight cache,
terminal grid view. Each arrives as its own crate in `crates/` only when the
component is real.
