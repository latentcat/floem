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
| accordion | `examples/widget-gallery/src/accordion.rs` | New | Single-open disclosure composition now covers open/closed trigger, disabled trigger, focus ring, border-separated items, chevron swap, wrapping trigger/body text, and shadcn content spacing. Reusable Accordion API and real height animation pending. |
| alert-dialog | `examples/widget-gallery/src/alert_dialog.rs` | New | Static shadcn-like alert dialog coverage now includes overlay preview, default/sm sizes, media and no-media headers, wrapping title/description, muted bordered footer, cancel/action buttons, and destructive action styling. Full modal/focus behavior pending. |
| alert | `examples/widget-gallery/src/alert.rs` | New | Default and destructive variants. |
| aspect-ratio | `examples/widget-gallery/src/aspect_ratio.rs` | New | Ratio examples using Floem aspect-ratio style. |
| attachment | `examples/widget-gallery/src/attachment.rs` | New | Horizontal/vertical attachments with sizes, actions, and idle/uploading/processing/error/done states. |
| avatar | `examples/widget-gallery/src/avatar.rs` | New | sm/default/lg, fallback, badge, group count. |
| badge | `examples/widget-gallery/src/badge.rs` | New | Variants: default, secondary, destructive, outline, ghost, link. |
| breadcrumb | `examples/widget-gallery/src/breadcrumb.rs` | New | Static shadcn-like breadcrumb coverage now includes nav/list/item/link/page slots, basic and collapsed paths, ellipsis, chevron and slash separators, icon root link, long wrapping path, current page styling, and link hover transition. |
| bubble | `examples/widget-gallery/src/bubble.rs` | New | Message bubble variants default/secondary/muted/tinted/outline/ghost/destructive plus reactions, with wrapped body text constrained inside the bubble surface. |
| button-group | `examples/widget-gallery/src/button_group.rs` | New | Horizontal, vertical, text, and separator compositions; text group now has left rounding and local List/Grid selection state. |
| button | `examples/widget-gallery/src/buttons.rs` / `src/style/theme.rs` | Partial | Core default radius updated to shadcn b0; gallery now covers default/secondary/outline/ghost/destructive/link, xs/sm/default/lg/icon sizes, inline icons, disabled, invalid, and expanded states. |
| calendar | `examples/widget-gallery/src/calendar.rs` | New | DayPicker-style calendar coverage now includes caption label/dropdown styling, nav buttons, weekday row, outside/today/selected/range-start/range-middle/range-end/focused/disabled/hidden day states, state samples, multi-month layout, and local selectable day state. Real DayPicker behavior pending. |
| card | `examples/widget-gallery/src/card.rs` | New | Default/sm spacing, header/content/footer basics. |
| carousel | `examples/widget-gallery/src/carousel.rs` | New | Static horizontal/vertical carousel layout with outline icon controls; Embla-like behavior pending. |
| chart | `examples/widget-gallery/src/chart.rs` | New | Static chart container with bars, legend, and tooltip surface; Recharts-like data API pending. |
| checkbox | `examples/widget-gallery/src/checkbox.rs` / `src/views/checkbox.rs` / `src/style/theme.rs` | Partial | Disabled no longer overrides checked primary fill; default checkmark moved to lucide-style stroke; gallery covers checked/unchecked, disabled checked/unchecked, invalid checked/unchecked, and labeled field composition. |
| collapsible | `examples/widget-gallery/src/collapsible.rs` | New | Disclosure composition now covers open, closed, and disabled trigger/content states with shadcn-like icon button styling and card surface. Reusable Collapsible API and height animation pending. |
| combobox | `examples/widget-gallery/src/combobox.rs` | New | Static shadcn-like combobox coverage now includes trigger/value/clear states, disabled and invalid triggers, input group, popover content, grouped list, labels, separator, highlighted/selected/disabled items, empty state, and chips invalid/disabled states. Real Base UI combobox behavior/filtering pending. |
| command | `examples/widget-gallery/src/command.rs` | New | Static shadcn-like command coverage now includes rounded-xl popover surface, input wrapper/group, max-height list area, groups, separator, selected/checked/disabled items, shortcuts, empty state, and dialog-style preview. Real cmdk filtering/keyboard behavior pending. |
| context-menu | `examples/widget-gallery/src/context_menu.rs` | Partial | Existing native popout/context actions retained; gallery visual surface now covers shadcn-like trigger/content, label, separator, icon/shortcut rows, disabled/destructive variants, checkbox/radio/inset rows, open sub trigger, and sub content. Native platform menu styling/focus fidelity still pending. |
| dialog | `examples/widget-gallery/src/dialog.rs` | New | Static shadcn-like dialog coverage now includes overlay preview, content ring/shadow, header/title/description, close button, form body, muted bordered footer, compact variant, and no-close-button state. Full modal/focus behavior pending. |
| direction | `examples/widget-gallery/src/direction.rs` | New | LTR/RTL layout-direction examples added; app-level direction provider API pending. |
| drawer | `examples/widget-gallery/src/drawer.rs` | New | Static shadcn-like drawer coverage now includes trigger/close controls, bottom/top/left/right directional content, bottom handle, header/title/description, body rows, footer actions, side-specific borders, and directional rounded corners. Real Vaul drawer behavior and motion pending. |
| dropdown-menu | `examples/widget-gallery/src/dropdown_menu.rs` | New | Static shadcn-style slot coverage now includes trigger/content, label, separator, icon items, shortcuts, disabled item, destructive item, checkbox/radio items, inset rows, open sub trigger, and sub content. Real menu overlay/focus/keyboard semantics pending. |
| empty | `examples/widget-gallery/src/empty_state.rs` | New | Empty layout with icon media and actions. |
| field | `examples/widget-gallery/src/field.rs` | New | Field labels, descriptions, separator, and error state. |
| hover-card | `examples/widget-gallery/src/hover_card.rs` | New | Tooltip-backed triggers and static shadcn-like hover-card surfaces now cover profile, repository, file, and team content with fixed avatar geometry, wrapping body text, and popover ring/shadow styling. Positioned HoverCard overlay API, open/close delay, and side-aware animation pending. |
| input-group | `examples/widget-gallery/src/input_group.rs` | New | Inline start/end, button, and block addon examples. |
| input-otp | `examples/widget-gallery/src/input_otp.rs` | New | Segmented slot coverage now includes grouped slots, separator, filled/default/active states, fake caret blink animation, invalid group ring, and disabled opacity state. Real input-otp behavior and keyboard navigation pending. |
| input | `examples/widget-gallery/src/inputs.rs` / `src/style/theme.rs` | Partial | Core input placeholder disabled styling cleaned up; gallery covers placeholder, filled, disabled, invalid, read-only, number, file-like, and search-with-icon states. |
| item | `examples/widget-gallery/src/item.rs` | New | default/outline/muted plus default/sm/xs sizing. |
| kbd | `examples/widget-gallery/src/kbd.rs` | New | Single key, key group, icon key. |
| label | `examples/widget-gallery/src/labels.rs` | Partial | Gallery updated for shadcn form-label states; core LabelClass audit pending. |
| marker | `examples/widget-gallery/src/marker.rs` | New | default, separator, and border variants. |
| menubar | `examples/widget-gallery/src/menubar.rs` | New | Static shadcn-like menubar coverage now includes root/trigger/content, active trigger, icon and inset items, label, separator, shortcut, disabled/destructive variants, checkbox/radio rows, open sub trigger, and sub content. Real menubar keyboard/menu behavior pending. |
| message-scroller | `examples/widget-gallery/src/message_scroller.rs` | New | Scroll viewport, content rows, and scroll-to-end button visual coverage. |
| message | `examples/widget-gallery/src/message.rs` | New | Start/end message layout with avatar, header, bubble content, and footer. |
| native-select | `examples/widget-gallery/src/native_select.rs` | New | Dropdown-backed select surface with default/sm sizes. |
| navigation-menu | `examples/widget-gallery/src/navigation_menu.rs` | New | Static shadcn-like navigation coverage now includes list/trigger states, disabled trigger, chevron rotation, indicator, shared viewport, viewport-disabled content, feature card, active/focusable links, and multiple content layouts. Real viewport measurement, keyboard behavior, and side-aware motion pending. |
| pagination | `examples/widget-gallery/src/pagination.rs` | New | Static shadcn-like pagination coverage now includes nav/content/item/link slots, active outline page, ghost pages, previous/next text buttons, compact icon-only controls, ellipsis, and disabled edge controls. |
| popover | `examples/widget-gallery/src/popover.rs` | New | Gallery now covers controlled visibility, shadcn content/header/description surfaces, form fields, action rows, status content, and top/right/bottom/left placement previews. Positioned overlay behavior and reusable Popover API pending. |
| progress | `examples/widget-gallery/src/progress.rs` | New | Static value examples with shadcn track/indicator styling. |
| radio-group | `examples/widget-gallery/src/radio_buttons.rs` / `src/style/theme.rs` | Partial | Disabled no longer overrides selected primary fill; gallery covers default, disabled, validation, selected invalid, and labeled radio rows. |
| resizable | `examples/widget-gallery/src/resizable.rs` / `src/views/resizable.rs` | Partial | Gallery added with real Resizable groups and shadcn-style handles; optional handle knob API pending. |
| scroll-area | `examples/widget-gallery/src/scroll_area.rs` | New | Vertical, horizontal, and both-axis examples use real Floem scroll with shadcn-like viewport focus ring, 10px scrollbar, rounded border thumb, transparent track, plus static viewport/scrollbar/corner anatomy. Exact Radix scrollbar visibility behavior pending. |
| select | `examples/widget-gallery/src/select.rs` / `src/views/dropdown.rs` / `src/style/theme.rs` | Partial | Dropdown-backed select now uses shadcn-like trigger/content/list-item base styling with 100ms color transitions and popover ring; gallery covers default/sm, placeholder, disabled, invalid, controlled open, and content anatomy with label/separator/checked/disabled item. Dropdown stale-focus guard added for closed overlays. Full Radix Select API, side-aware animation, and typed reusable Select wrapper pending. |
| separator | `examples/widget-gallery/src/separator.rs` | New | Horizontal and vertical orientations. |
| sheet | `examples/widget-gallery/src/sheet.rs` | New | Static shadcn-like sheet coverage now includes trigger/close controls, right/left/top/bottom side variants, side-specific borders, header/title/description, close button optional state, body fields, footer actions, and overlay-framed previews. Real overlay/motion pending. |
| sidebar | Widget gallery sidebar | Partial | Gallery sidebar styled with shadcn tokens; active page content now loads through `dyn_view` and disposes old pages on switch. Reusable component pending. |
| skeleton | `examples/widget-gallery/src/skeleton.rs` | New | Pulse opacity animation added. |
| slider | `examples/widget-gallery/src/slider.rs` / `src/views/slider.rs` / `src/style/theme.rs` | Partial | Core slider now supports vertical orientation and disabled event blocking; gallery covers default, step, ranged value, disabled, vertical, readonly progress, and static multi-thumb range preview; real multi-thumb API and thumb ring stroke pending. |
| sonner | `examples/widget-gallery/src/sonner.rs` | New | Static toast variants with shadcn popover styling and lucide status icons; real toast system pending. |
| spinner | `examples/widget-gallery/src/spinner.rs` | New | Lucide loader-circle with sizes and multi-step linear repeat keyframes for continuous rotation. |
| switch | `examples/widget-gallery/src/switch.rs` / `src/views/toggle_button.rs` / `src/style/theme.rs` | Partial | ToggleButton now supports checked/unchecked handle insets for shadcn thumb geometry; default/sm, checked/unchecked, disabled, invalid, and interactive states covered in gallery. |
| table | `examples/widget-gallery/src/table.rs` | New | Header/body/footer/caption-style example. |
| tabs | `examples/widget-gallery/src/tabs.rs` / `src/style/theme.rs` | Partial | Tab selector selected state now has shadcn-like shadow and not-allowed disabled cursor; gallery covers fit-content default/line/icon layouts, full-width proportional/equal trigger distribution, disabled trigger, and vertical line orientation. |
| textarea | `examples/widget-gallery/src/textarea.rs` | New | Textarea-styled TextInput surface with invalid/disabled states. |
| toggle-group | `examples/widget-gallery/src/toggle_group.rs` | New | Single-select default/outline/joined/vertical groups. |
| toggle | `examples/widget-gallery/src/toggle.rs` | New | Default/outline, pressed state, and size examples. |
| tooltip | `examples/widget-gallery/src/tooltip.rs` / `src/views/tooltip.rs` / `src/style/theme.rs` | Partial | Core tooltip overlay now renders a shadcn-like arrow; TooltipClass max width added; gallery covers text/icon/rich triggers plus static arrow/kbd/icon surfaces. Animation and side-aware placement still pending. |

## Current Batch

- Added built-in icon assets and compile-time icon index.
- Added Icon gallery page.
- Added Badge, Card, Separator, and Skeleton gallery pages.
- Added Alert, Avatar, Breadcrumb, Kbd, Progress, and Table gallery pages.
- Added Empty, Item, Marker, Pagination, Spinner, and Button Group gallery pages.
- Added Aspect Ratio, Field, Input Group, Native Select, Textarea, and Toggle gallery pages.
- Updated Checkbox, Radio Group, Input, Slider, and Tabs toward shadcn default state styling.
- Added Scroll Area gallery page.
- Updated Tooltip and Dropdown/Select base styling toward shadcn.
- Added Select, Tooltip, Toggle Group, Collapsible, and updated Label gallery pages.
- Added Accordion, Dropdown Menu, Popover, and Hover Card gallery pages.
- Updated Context Menu gallery with shadcn-style visual surface.
- Added Dialog, Alert Dialog, Sheet, Drawer, Command, Combobox, and Input OTP gallery pages.
- Added Direction, Resizable, Sonner, Menubar, and Navigation Menu gallery pages.
- Added Attachment, Bubble, Message, Message Scroller, Calendar, Carousel, and Chart gallery pages.
- Updated Button default style and gallery coverage for shadcn b0 variants, sizes, icon buttons, and key states.
- Updated Switch geometry, transition timing, small size, invalid/disabled states, and gallery coverage toward shadcn b0.
- Updated Input placeholder disabled styling and gallery coverage for core shadcn b0 input states.
- Updated Checkbox and Radio Group disabled/invalid state handling and gallery coverage toward shadcn b0.
- Updated Slider vertical orientation support, disabled interaction behavior, ranged examples, and gallery coverage toward shadcn b0.
- Updated Tabs selector active/disabled styling and gallery coverage for default, line, icons, disabled, and vertical orientations.
- Updated Tooltip arrow rendering, max-width styling, and gallery coverage for text, icon, rich, kbd, and static surfaces.
- Updated Dropdown/Select base styling and Select gallery coverage for default/sm, placeholder, disabled, invalid, controlled open, and content anatomy states.
- Expanded Popover gallery coverage for controlled visibility, header/description content, forms, action rows, status content, and placement surfaces.
- Expanded Dropdown Menu gallery into shadcn-like slots with trigger/content, label/separator, icon/shortcut rows, checkbox/radio/inset states, disabled/destructive variants, and submenu previews.
- Expanded Hover Card gallery with tooltip-backed triggers and profile/repository/file/team hover-card surfaces using shadcn popover ring/shadow styling.
- Updated Scroll Area gallery with shadcn-like focus ring, themed thumb/track styling, vertical/horizontal/both-axis examples, and viewport/scrollbar/corner anatomy.
- Expanded Context Menu gallery into shadcn-like slots while retaining Floem native popout/context menu action demos.
- Expanded Menubar gallery with shadcn-like root/trigger/content, active trigger, label/separator/shortcut, disabled/destructive, checkbox/radio, inset, and submenu states.
- Expanded Navigation Menu gallery with shadcn-like trigger/list/indicator/viewport states, disabled trigger, chevron rotation, active links, feature card, and viewport-disabled content.
- Expanded Command gallery with shadcn-like command surface, input wrapper, list/groups/separator, selected/checked/disabled items, shortcuts, empty state, and dialog preview.
- Expanded Combobox gallery with shadcn-like trigger/value/clear, input group, popover content, grouped list, highlighted/selected/disabled items, empty state, chips, invalid, and disabled states.
- Expanded Input OTP gallery with grouped slots, separator, active/filled/default states, fake caret blink animation, invalid ring, and disabled state.
- Expanded Dialog and Alert Dialog gallery coverage with shadcn-like overlay/content, title/description, close/media/no-media states, muted bordered footers, compact/default variants, and action rows.
- Expanded Sheet gallery with shadcn-like side variants, side-specific borders, header/body/footer slots, close optional state, and overlay-framed previews.
- Expanded Drawer gallery with shadcn-like trigger/close controls, direction-specific content, bottom handle, header/body/footer slots, side borders, and rounded directional corners.
- Updated Accordion and Collapsible gallery coverage for open/closed/disabled states, focus/opacity handling, shadcn spacing, and disclosure trigger styling.
- Expanded Pagination and Breadcrumb gallery coverage with shadcn-like slot structure, active/current states, compact/collapsed variants, ellipsis, separators, icons, and disabled controls.
- Expanded Calendar gallery with shadcn-like caption/dropdown, nav buttons, focused day, full range states, disabled/hidden/outside/today states, and multi-month layout.
- Updated gallery active page rendering from eager `tab` mounting to lazy `dyn_view` loading/unloading, and tightened Alert Dialog/Bubble wrapping, Spinner loop easing, Button Group selection, and Calendar local interaction.
- Added shared gallery shadcn text/fixed-square style helpers; tightened Accordion/Hover Card/Dialog text wrapping and fixed icon/avatar shrink; expanded Tabs layout variants; guarded Dropdown stale overlay focus; updated Spinner to multi-step linear rotation.
