# Shadcn Component Alignment Status

Reference project: `references/shadcn-next`

Goal: maximize component coverage first, then tighten pixel-level details after the full surface exists.

Status legend:
- `Aligned`: implemented in Floem/gallery with shadcn-style variants and basic states.
- `Partial`: exists, but variants, states, animation, or exact sizing still need work.
- `New`: simple shadcn component added in gallery; may still need extraction into reusable Floem APIs.
- `Missing`: not implemented yet.
- `Later`: intentionally deferred because behavior is more complex.

| Component | Floem / Gallery Surface | Status | Notes |
| --- | --- | --- | --- |
| accordion | None | Later | Needs disclosure state and animation. |
| alert-dialog | None | Later | Needs modal/dialog primitives. |
| alert | None | Missing | Simple static component; next candidate. |
| aspect-ratio | layout/style primitives | Missing | Could add a page using aspect ratio style. |
| attachment | None | Later | Domain-specific. |
| avatar | None | Missing | Simple next candidate. |
| badge | `examples/widget-gallery/src/badge.rs` | New | Variants: default, secondary, destructive, outline, ghost, link. |
| breadcrumb | None | Missing | Simple text/icon composition. |
| bubble | None | Later | Message UI composition. |
| button-group | Button primitives | Missing | Needs grouped button radii/borders. |
| button | `examples/widget-gallery/src/buttons.rs` | Partial | Variants and sizes started; needs deeper state audit. |
| calendar | None | Later | Larger behavior surface. |
| card | `examples/widget-gallery/src/card.rs` | New | Default/sm spacing, header/content/footer basics. |
| carousel | None | Later | Larger behavior surface. |
| chart | None | Later | External chart semantics. |
| checkbox | `examples/widget-gallery/src/checkbox.rs` | Partial | Existing Floem component; shadcn state/style audit pending. |
| collapsible | None | Later | Needs disclosure state and animation. |
| combobox | Dropdown/TextInput primitives | Later | Needs command/search behavior. |
| command | None | Later | Larger behavior surface. |
| context-menu | `examples/widget-gallery/src/context_menu.rs` | Partial | Existing; shadcn style audit pending. |
| dialog | Overlay primitives | Later | Needs modal surface. |
| direction | None | Missing | Likely utility/demo only. |
| drawer | None | Later | Needs overlay + motion. |
| dropdown-menu | Dropdown/context primitives | Later | Needs menu semantics. |
| empty | Empty primitive | Missing | Add shadcn empty composition. |
| field | None | Missing | Form layout component. |
| hover-card | Tooltip/overlay primitives | Later | Needs hover overlay. |
| input-group | TextInput primitives | Missing | Needs composition styles. |
| input-otp | None | Later | Needs input segmentation. |
| input | `examples/widget-gallery/src/inputs.rs` | Partial | Existing Floem component; shadcn style audit pending. |
| item | None | Missing | Simple composition. |
| kbd | None | Missing | Simple static component. |
| label | `examples/widget-gallery/src/labels.rs` | Partial | Existing label; shadcn form label audit pending. |
| marker | None | Missing | Simple static component. |
| menubar | None | Later | Menu behavior. |
| message-scroller | None | Later | Domain-specific. |
| message | None | Later | Domain-specific. |
| native-select | Dropdown primitives | Missing | Native select equivalent pending. |
| navigation-menu | None | Later | Complex menu behavior. |
| pagination | None | Missing | Button/link composition. |
| popover | Overlay primitives | Later | Needs positioned overlay API. |
| progress | Slider/custom primitives | Missing | Simple next candidate. |
| radio-group | `examples/widget-gallery/src/radio_buttons.rs` | Partial | Existing Floem component; shadcn state/style audit pending. |
| resizable | `examples/widget-gallery/src/*resizable*` / core | Partial | Existing core support; gallery surfacing audit pending. |
| scroll-area | Scroll primitives | Partial | Existing scroll styling; shadcn page pending. |
| select | Dropdown primitives | Later | Needs select semantics and shadcn styling. |
| separator | `examples/widget-gallery/src/separator.rs` | New | Horizontal and vertical orientations. |
| sheet | None | Later | Needs overlay + motion. |
| sidebar | Widget gallery sidebar | Partial | Gallery sidebar styled with shadcn tokens; reusable component pending. |
| skeleton | `examples/widget-gallery/src/skeleton.rs` | New | Pulse opacity animation added. |
| slider | `examples/widget-gallery/src/slider.rs` | Partial | Existing Floem component; shadcn state/style audit pending. |
| sonner | None | Later | Toast system. |
| spinner | None | Missing | Simple animated icon/loader candidate. |
| switch | `examples/widget-gallery/src/switch.rs` | Partial | Basic shadcn size/color/state done; more audit pending. |
| table | None | Missing | Simple static/table composition candidate. |
| tabs | `examples/widget-gallery/src/tabs.rs` | Partial | Existing Floem component; shadcn style audit pending. |
| textarea | TextEditor/TextInput primitives | Missing | Needs multiline input surface. |
| toggle-group | None | Later | Needs toggle group state. |
| toggle | Button primitives | Missing | Simple pressed state candidate. |
| tooltip | Tooltip primitives | Partial | Existing component; shadcn style audit pending. |

## Current Batch

- Added built-in icon assets and compile-time icon index.
- Added Icon gallery page.
- Added Badge, Card, Separator, and Skeleton gallery pages.
