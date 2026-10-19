# Balatro Joker Calculator — session bundle (2026-10-18)

Everything produced in this session, as files.

```
balatro-joker-calculator/
├── README.md
├── design/
│   └── project/
│       ├── canvas.json          # Design canvas index: two artboards, frames, flags
│       ├── Main.dc.html         # Calculator page — interactive (hand type, joker slots, breakdown)
│       └── JokerCard.dc.html    # Simplified joker card anatomy + effect variants (static)
└── docs/
    ├── EPIC-05_Joker_Calculator.md   # The integration epic (cardpack.rs funky ↔ calculator)
    └── architecture.svg              # Standalone render of the epic's architecture diagram
```

## Design files

The `.dc.html` files are Design Component pages: a self-contained HTML document whose UI is the
`<x-dc>` markup and whose logic is the `class Component extends DCLogic` script. They reference
`./support.js`, which the Design canvas runtime provides — opened directly in a browser they will
render the markup but the live calculator logic (state, handlers) needs that runtime.

`Main.dc.html` carries the current scoring model in plain JavaScript (`HANDS`, `JOKERS`, and the
step loop in `renderVals()`). Per the epic, that JS is the part slated to be replaced by a call into
`cardpack::funky` through a wasm adapter.

## The epic

`docs/EPIC-05_Joker_Calculator.md` is written in the format of cardpack.rs's existing
`docs/EPIC-*.md` files and is intended to be dropped in there as-is. The architecture diagram is a
Mermaid block (renders on GitHub); `architecture.svg` is the same picture as a standalone file.

Open question to settle before Story 1: `Score.mult` is a `usize` with ceiling after each ×mult in
the kernel, while the calculator keeps fractional mult.
