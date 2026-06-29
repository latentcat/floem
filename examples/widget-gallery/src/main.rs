pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod animation;
pub mod aspect_ratio;
pub mod attachment;
pub mod avatar;
pub mod background_blur;
pub mod badge;
pub mod bevy_render_test;
pub mod breadcrumb;
pub mod bubble;
pub mod button_group;
pub mod buttons;
pub mod calendar;
pub mod canvas;
pub mod card;
pub mod carousel;
pub mod chart;
pub mod checkbox;
pub mod clipboard;
pub mod collapsible;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod corner_smoothing;
pub mod dialog;
pub mod direction;
pub mod draggable;
pub mod drawer;
pub mod dropdown;
pub mod dropdown_menu;
pub mod dropped_file;
pub mod empty_state;
pub mod field;
pub mod form;
pub mod hover_card;
pub mod icons;
pub mod images;
pub mod input_group;
pub mod input_otp;
pub mod inputs;
pub mod item;
pub mod kbd;
pub mod labels;
pub mod lists;
pub mod marker;
pub mod menubar;
pub mod message;
pub mod message_scroller;
pub mod native_select;
pub mod navigation_menu;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio_buttons;
pub mod render_test_common;
pub mod resizable;
pub mod rich_text;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod shadcn_style;
pub mod sheet;
pub mod skeleton;
pub mod slider;
pub mod sonner;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod texteditor;
pub mod toggle;
pub mod toggle_group;
pub mod tooltip;
pub mod wgpu_render_test;

use floem::{
    action::{set_theme, set_window_menu, toggle_global_theme, toggle_window_theme},
    kurbo::Size,
    menu::*,
    muda::{AboutMetadataBuilder, PredefinedMenuItem},
    new_window,
    peniko::Color,
    prelude::*,
    style::{Background, CursorStyle, CustomStylable, Transition},
    theme::StyleThemeExt,
    ui_events::keyboard::{Key, KeyboardEvent, Modifiers, NamedKey},
    window::{Theme, WindowConfig, WindowId},
};

pub const OS_MOD: Modifiers = if cfg!(target_os = "macos") {
    Modifiers::META
} else {
    Modifiers::CONTROL
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum GalleryThemeMode {
    System,
    Light,
    Dark,
}

fn sidebar_button_style() -> floem::style::Style {
    floem::style::Style::new()
        .height(32.0)
        .padding_horiz(12.0)
        .background(Color::TRANSPARENT)
        .border_color(Color::TRANSPARENT)
        .with_theme(|s, t| {
            s.color(t.sidebar_foreground())
                .hover(|s| {
                    s.background(t.sidebar_accent())
                        .color(t.sidebar_accent_foreground())
                })
                .focus_visible(|s| s.outline_color(t.def(|t| t.sidebar_ring().with_alpha(0.3))))
        })
}

fn theme_mode_button(
    label: &'static str,
    mode: GalleryThemeMode,
    theme_mode: RwSignal<GalleryThemeMode>,
) -> Button {
    Button::new(label)
        .action(move || {
            theme_mode.set(mode);
            match mode {
                GalleryThemeMode::System => set_theme(None),
                GalleryThemeMode::Light => set_theme(Some(Theme::Light)),
                GalleryThemeMode::Dark => set_theme(Some(Theme::Dark)),
            }
        })
        .style(move |s| {
            s.height(28.0)
                .padding_horiz(10.0)
                .background(Color::TRANSPARENT)
                .border_color(Color::TRANSPARENT)
                .with_theme(|s, t| {
                    s.color(t.sidebar_foreground()).hover(|s| {
                        s.background(t.sidebar_accent())
                            .color(t.sidebar_accent_foreground())
                    })
                })
                .apply_if(theme_mode.get() == mode, |s| {
                    s.with_theme(|s, t| {
                        s.background(t.sidebar_accent())
                            .color(t.sidebar_accent_foreground())
                    })
                })
        })
}

fn app_view(window_id: WindowId) -> impl IntoView {
    let theme_mode = RwSignal::new(GalleryThemeMode::System);
    let tabs: Vec<&'static str> = vec![
        "Label",
        "Button",
        "Accordion",
        "Badge",
        "Alert",
        "Alert Dialog",
        "Attachment",
        "Avatar",
        "Aspect Ratio",
        "Breadcrumb",
        "Bubble",
        "Button Group",
        "Calendar",
        "Carousel",
        "Combobox",
        "Command",
        "Collapsible",
        "Context Menu",
        "Dialog",
        "Direction",
        "Drawer",
        "Dropdown Menu",
        "Empty",
        "Field",
        "Hover Card",
        "Input OTP",
        "Input Group",
        "Item",
        "Kbd",
        "Marker",
        "Menubar",
        "Message",
        "Message Scroller",
        "Native Select",
        "Navigation Menu",
        "Pagination",
        "Popover",
        "Switch",
        "Icon",
        "Card",
        "Chart",
        "Scroll Area",
        "Select",
        "Separator",
        "Sheet",
        "Skeleton",
        "Progress",
        "Resizable",
        "Spinner",
        "Sonner",
        "Table",
        "Textarea",
        "Toggle",
        "Toggle Group",
        "Tooltip",
        "Input",
        "Text Editor",
        "Lists",
        "Image",
        "Background Blur",
        "Corner Smoothing",
        "WGPU Render Test",
        "Bevy Render Test",
        "Dropdown",
        "Checkbox",
        "Radio",
        "Tabs",
        "Slider",
        "Canvas",
        "Menu",
        "Rich Text",
        "Clipboard",
        "Animation",
        "Draggable",
        "Dropped File",
        "Files",
    ];

    let create_view = |it: &str| {
        match it {
            "Label" => labels::label_view().into_any(),
            "Button" => buttons::button_view().into_any(),
            "Accordion" => accordion::accordion_view().into_any(),
            "Badge" => badge::badge_view().into_any(),
            "Alert" => alert::alert_view().into_any(),
            "Alert Dialog" => alert_dialog::alert_dialog_view().into_any(),
            "Attachment" => attachment::attachment_view().into_any(),
            "Avatar" => avatar::avatar_view().into_any(),
            "Aspect Ratio" => aspect_ratio::aspect_ratio_view().into_any(),
            "Breadcrumb" => breadcrumb::breadcrumb_view().into_any(),
            "Bubble" => bubble::bubble_view().into_any(),
            "Button Group" => button_group::button_group_view().into_any(),
            "Calendar" => calendar::calendar_view().into_any(),
            "Carousel" => carousel::carousel_view().into_any(),
            "Combobox" => combobox::combobox_view().into_any(),
            "Command" => command::command_view().into_any(),
            "Collapsible" => collapsible::collapsible_view().into_any(),
            "Context Menu" => context_menu::menu_view().into_any(),
            "Dialog" => dialog::dialog_view().into_any(),
            "Direction" => direction::direction_view().into_any(),
            "Drawer" => drawer::drawer_view().into_any(),
            "Dropdown Menu" => dropdown_menu::dropdown_menu_view().into_any(),
            "Empty" => empty_state::empty_view().into_any(),
            "Field" => field::field_view().into_any(),
            "Hover Card" => hover_card::hover_card_view().into_any(),
            "Input OTP" => input_otp::input_otp_view().into_any(),
            "Input Group" => input_group::input_group_view().into_any(),
            "Item" => item::item_view().into_any(),
            "Kbd" => kbd::kbd_view().into_any(),
            "Marker" => marker::marker_view().into_any(),
            "Menubar" => menubar::menubar_view().into_any(),
            "Message" => message::message_view().into_any(),
            "Message Scroller" => message_scroller::message_scroller_view().into_any(),
            "Native Select" => native_select::native_select_view().into_any(),
            "Navigation Menu" => navigation_menu::navigation_menu_view().into_any(),
            "Pagination" => pagination::pagination_view().into_any(),
            "Popover" => popover::popover_view().into_any(),
            "Switch" => switch::switch_view().into_any(),
            "Icon" => icons::icons_view().into_any(),
            "Card" => card::card_view().into_any(),
            "Chart" => chart::chart_view().into_any(),
            "Scroll Area" => scroll_area::scroll_area_view().into_any(),
            "Select" => select::select_view().into_any(),
            "Separator" => separator::separator_view().into_any(),
            "Sheet" => sheet::sheet_view().into_any(),
            "Skeleton" => skeleton::skeleton_view().into_any(),
            "Progress" => progress::progress_view().into_any(),
            "Resizable" => resizable::resizable_view().into_any(),
            "Spinner" => spinner::spinner_view().into_any(),
            "Sonner" => sonner::sonner_view().into_any(),
            "Table" => table::table_view().into_any(),
            "Textarea" => textarea::textarea_view().into_any(),
            "Toggle" => toggle::toggle_view().into_any(),
            "Toggle Group" => toggle_group::toggle_group_view().into_any(),
            "Tooltip" => tooltip::tooltip_view().into_any(),
            "Checkbox" => checkbox::checkbox_view().into_any(),
            "Radio" => radio_buttons::radio_buttons_view().into_any(),
            "Input" => inputs::text_input_view().into_any(),
            "Canvas" => canvas::canvas_view().into_any(),
            "Lists" => lists::list_view().into_any(),
            "Tabs" => tabs::tab_view().into_any(),
            "Menu" => context_menu::menu_view().into_any(),
            "Rich Text" => rich_text::rich_text_view().into_any(),
            "Image" => images::img_view().into_any(),
            "Background Blur" => background_blur::background_blur_view().into_any(),
            "Corner Smoothing" => corner_smoothing::corner_smoothing_view().into_any(),
            "WGPU Render Test" => wgpu_render_test::wgpu_render_test_view().into_any(),
            "Bevy Render Test" => bevy_render_test::bevy_render_test_view().into_any(),
            "Clipboard" => clipboard::clipboard_view().into_any(),
            "Slider" => slider::slider_view().into_any(),
            "Dropdown" => dropdown::dropdown_view().into_any(),
            "Animation" => animation::animation_view().into_any(),
            "Draggable" => draggable::draggable_view().into_any(),
            "Dropped File" => dropped_file::dropped_file_view().into_any(),
            #[cfg(feature = "full")]
            "Files" => files::files_view().into_any(),
            "Text Editor" => texteditor::editor_view().into_any(),
            _ => Label::derived(|| "Not implemented".to_owned()).into_any(),
        }
        .debug_name(it.to_string())
    };

    let tabs = RwSignal::new(tabs);

    let side_bar_list = tabs
        .get()
        .into_iter()
        .map(move |item| {
            item.style(move |s| {
                s.font_size(14.)
                    .height(32.0)
                    .width_full()
                    .padding_horiz(10.0)
                    .items_center()
                    .transition(Background, Transition::ease_in_out(100.millis()))
                    .with_theme(|s, t| s.color(t.sidebar_foreground()))
                    .active(|s| {
                        s.with_theme(|s, t| {
                            s.background(t.sidebar_accent())
                                .color(t.sidebar_foreground())
                                .hover(|s| s.background(t.sidebar_accent()))
                                .border_radius(t.border_radius())
                        })
                    })
                    .selected(|s| {
                        s.with_theme(|s, t| {
                            s.background(t.sidebar_accent())
                                .color(t.sidebar_foreground())
                                .hover(|s| s.background(t.sidebar_accent()))
                                .border_radius(t.border_radius())
                        })
                    })
                    .hover(|s| {
                        s.cursor(CursorStyle::Pointer).with_theme(|s, t| {
                            s.background(t.sidebar_accent())
                                .color(t.sidebar_foreground())
                        })
                    })
            })
        })
        .list()
        .style(|s| {
            s.flex_col()
                .width_full()
                .flex_grow(1.)
                .gap(2.0)
                .class(ListItemClass, |s| {
                    s.selected(|s| {
                        s.with_theme(|s, t| {
                            s.background(t.sidebar_accent())
                                .color(t.sidebar_foreground())
                                .hover(|s| s.background(t.sidebar_accent()))
                        })
                    })
                })
        });

    let active_tab = side_bar_list.selection();

    let side_tab_bar = side_bar_list
        .scroll()
        .debug_name("Side Tab Bar")
        .custom_style(|s| s.shrink_to_fit())
        .style(|s| {
            s.flex_col()
                .width_full()
                .flex_grow(1.0)
                .padding(8.0)
                .with_theme(|s, t| s.background(t.sidebar()).color(t.sidebar_foreground()))
                .class(LabelClass, |s| s.selectable(false))
        });

    let theme_mode_control = Stack::horizontal((
        theme_mode_button("System", GalleryThemeMode::System, theme_mode),
        theme_mode_button("Light", GalleryThemeMode::Light, theme_mode),
        theme_mode_button("Dark", GalleryThemeMode::Dark, theme_mode),
    ))
    .style(|s| {
        s.width_full()
            .items_center()
            .gap(2.0)
            .padding(2.0)
            .border(1.0)
            .border_radius(14.0)
            .with_theme(|s, t| s.border_color(t.sidebar_border()))
    });

    let inspector = Button::new("Open Inspector")
        .action(floem::action::inspect)
        .style(|s| s.width_full().justify_start().apply(sidebar_button_style()));

    let new_window_button = Button::new("Open In Window")
        .action(move || {
            let name = tabs.with(|tabs| tabs.get(active_tab.get().unwrap_or(0)).copied());
            new_window(
                move |_| {
                    create_view(name.unwrap_or_default())
                        .scroll()
                        .style(|s| s.size_full())
                },
                Some(
                    WindowConfig::default()
                        .size(Size::new(700.0, 400.0))
                        .title(name.unwrap_or_default()),
                ),
            );
        })
        .style(|s| s.width_full().justify_start().apply(sidebar_button_style()));

    let separator = Empty::new().style(|s| {
        s.width_full()
            .height(1.0)
            .flex_shrink(0.0)
            .with_theme(|s, t| s.background(t.sidebar_border()))
    });

    let fixed_sidebar_controls =
        Stack::vertical((theme_mode_control, new_window_button, inspector))
            .style(|s| s.width_full().padding(8.0).row_gap(8.0).flex_shrink(0.0));

    let left_side_bar = Stack::vertical((side_tab_bar, separator, fixed_sidebar_controls))
        .debug_name("Left Side Bar")
        .style(|s| {
            s.height_full()
                .width(216.0)
                .border_right(1.0)
                .with_theme(|s, t| {
                    s.background(t.sidebar())
                        .color(t.sidebar_foreground())
                        .border_color(t.sidebar_border())
                })
        });

    let tab = dyn_view(move || {
        let name = tabs.with(|tabs| {
            tabs.get(active_tab.get().unwrap_or(0))
                .copied()
                .unwrap_or("Label")
        });
        create_view(name)
    })
    .debug_name("Active Tab")
    .style(|s| s.flex_col().flex_grow(1.).items_start());

    let tab = tab.scroll().style(|s| {
        s.size_full()
            .padding(12.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    });

    let floem_logo = svg(include_str!("../assets/floem.svg"))
        .style(|s| s.unset_color().size_full().size(50, 50))
        .overlay()
        .style(|s| {
            s.absolute()
                .z_index(1)
                .size(50, 50)
                .inset_bottom(20.)
                .inset_right(15.)
        });

    let view = Stack::horizontal((left_side_bar, tab, floem_logo))
        .style(|s| {
            s.width_full()
                .height_full()
                .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
        })
        .window_title(|| "Widget Gallery".to_owned());

    let file_submenu = |m: SubMenu| {
        m.item("New Window", |i| {
            i.action(move || {
                new_window(app_view, None);
            })
        })
        .separator()
        .item("Close Window", |i| {
            i.action(move || {
                floem::close_window(window_id);
            })
        })
        .item("Quit Widget Gallery", |i| {
            i.action(|| {
                floem::quit_app();
            })
        })
    };

    let widget_submenu = |m: SubMenu| {
        tabs.with(|tabs| {
            tabs.iter().enumerate().fold(m, |menu, (idx, &tab)| {
                menu.item(tab, move |i| i.action(move || active_tab.set(Some(idx))))
            })
        })
    };

    let view_submenu = |m: SubMenu| {
        m.item("Inspector", |i| {
            i.action(|| {
                floem::action::inspect();
            })
        })
        .separator()
        .submenu("Navigate to Widget", widget_submenu)
        .separator()
        .item("Next Tab", |i| {
            i.action(move || {
                let current = active_tab.get().unwrap_or(0);
                let tab_count = tabs.get().len();
                active_tab.set(Some((current + 1) % tab_count));
            })
        })
        .item("Previous Tab", |i| {
            i.action(move || {
                let current = active_tab.get().unwrap_or(0);
                let tab_count = tabs.get().len();
                active_tab.set(if current == 0 {
                    Some(tab_count - 1)
                } else {
                    Some(current - 1)
                });
            })
        })
    };

    let theme_submenu = |m: SubMenu| {
        m.item("Toggle Global Theme", |i| i.action(toggle_global_theme))
            .item("Toggle Window Theme", |i| i.action(toggle_window_theme))
            .separator()
            .item("Set Light Theme", |i| {
                i.action(|| set_theme(Some(Theme::Light)))
            })
            .item("Set Dark Theme", |i| {
                i.action(|| set_theme(Some(Theme::Dark)))
            })
            .item("Follow OS Theme", |i| i.action(|| set_theme(None)))
    };

    let window_submenu = |m: SubMenu| {
        m.item("Open Current Tab in New Window", |i| {
            i.action(move || {
                let name = tabs.with(|tabs| tabs.get(active_tab.get().unwrap_or(0)).copied());
                new_window(
                    move |_| {
                        create_view(name.unwrap_or_default())
                            .scroll()
                            .style(|s| s.size_full())
                    },
                    Some(
                        WindowConfig::default()
                            .size(Size::new(700.0, 400.0))
                            .title(name.unwrap_or_default()),
                    ),
                );
            })
        })
        .separator()
        .item("Show Side Panel", |i| {
            i.checked(true).action(|| {
                println!("Toggle sidebar");
            })
        })
    };

    let help_submenu = |m: SubMenu| {
        m.item("About Widget Gallery", |i| {
            i.action(|| {
                println!("Floem Widget Gallery - A showcase of UI components built with Floem");
            })
        })
        .separator()
        .item("Floem Documentation", |i| {
            i.action(|| {
                println!("Opening Floem documentation...");
            })
        })
        .item("GitHub Repository", |i| {
            i.action(|| {
                println!("Opening GitHub repository...");
            })
        })
    };
    set_window_menu(
        Menu::new()
            .submenu("File", file_submenu)
            .submenu("View", view_submenu)
            .submenu("Window", window_submenu)
            .submenu("Theme", theme_submenu)
            .submenu("Help", help_submenu)
            .submenu("About", |s| {
                s.predefined(&PredefinedMenuItem::about(
                    Some("widget-gallery"),
                    Some(
                        AboutMetadataBuilder::new()
                            .name(Some("widget-gallery"))
                            .license(Some("MIT"))
                            .version(Some("0.1.0"))
                            .copyright(Some("Floem Authors"))
                            .build(),
                    ),
                ))
            }),
    );

    let mut window_scale = RwSignal::new(1.);

    view.on_event_stop(
        listener::KeyUp,
        move |_cx, KeyboardEvent { modifiers, key, .. }| {
            if *key == Key::Named(NamedKey::F11) {
                floem::action::inspect();
            } else if *key == Key::Character("q".into()) && modifiers.contains(OS_MOD) {
                floem::quit_app();
            } else if *key == Key::Character("w".into()) && modifiers.contains(OS_MOD) {
                floem::close_window(window_id);
            }
        },
    )
    .on_event_stop(
        el::KeyDown,
        move |_, KeyboardEvent { key, modifiers, .. }| match key {
            Key::Character(ch) if (ch == "=" || ch == "+") && modifiers.contains(OS_MOD) => {
                window_scale *= 1.1;
                floem::action::set_window_scale(window_scale.get());
            }

            Key::Character(ch) if ch == "-" && *modifiers == OS_MOD => {
                window_scale /= 1.1;
                floem::action::set_window_scale(window_scale.get());
            }

            Key::Character(ch) if ch == "0" && *modifiers == OS_MOD => {
                window_scale.set(1.);
                floem::action::set_window_scale(window_scale.get());
            }
            _ => {}
        },
    )
}

fn main() {
    floem::Application::new()
        .window(app_view, Some(WindowConfig::default().size((1200., 800.))))
        .on_event(|ae| match ae {
            floem::AppEvent::WillTerminate => {
                println!("terminating");
            }
            floem::AppEvent::Reopen {
                has_visible_windows,
            } => {
                if !has_visible_windows {
                    new_window(app_view, None);
                }
            }
        })
        .run();
}
