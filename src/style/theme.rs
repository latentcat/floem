use std::time::Duration;

use super::unit::{DurationUnitExt, UnitExt};
use super::*;
use crate::style::Selectable;
use crate::view::View;
use crate::views::editor::SelectionColor;
use crate::views::resizable::{ResizableClass, ResizableHandleClass};
use crate::{
    AnyView, prop, style_class, style_debug_group,
    views::{
        ButtonClass, CheckboxClass, LabelClass, LabelCustomExprStyle, LabelCustomStyle,
        LabeledCheckboxClass, LabeledRadioButtonClass, ListClass, ListItemClass,
        PlaceholderTextClass, RadioButtonClass, RadioButtonDotClass, SvgClass, TabSelectorClass,
        TextInputClass, ToggleButtonCheckedInset, ToggleButtonCircleRad, ToggleButtonClass,
        ToggleButtonInset, ToggleButtonUncheckedInset, TooltipClass, dropdown,
        resizable::{ResizableCustomExprStyle, ResizableCustomStyle},
        scroll,
        slider::{SliderClass, SliderCustomExprStyle, SliderCustomStyle},
    },
};
use floem_renderer::text::FontWeight;
use peniko::{Brush, Color, color::palette::css};
use smallvec::smallvec;

style_class!(pub HoverTargetClass);

fn border_debug_view(style: &Style) -> Option<Box<dyn View>> {
    Border {
        left: style.get_prop::<BorderLeft>(),
        top: style.get_prop::<BorderTop>(),
        right: style.get_prop::<BorderRight>(),
        bottom: style.get_prop::<BorderBottom>(),
    }
    .debug_view()
}

fn border_color_debug_view(style: &Style) -> Option<Box<dyn View>> {
    BorderColor {
        left: style.get_prop::<BorderLeftColor>().flatten(),
        top: style.get_prop::<BorderTopColor>().flatten(),
        right: style.get_prop::<BorderRightColor>().flatten(),
        bottom: style.get_prop::<BorderBottomColor>().flatten(),
    }
    .debug_view()
}

fn border_radius_debug_view(style: &Style) -> Option<Box<dyn View>> {
    BorderRadius {
        top_left: style.get_prop::<BorderTopLeftRadius>(),
        top_right: style.get_prop::<BorderTopRightRadius>(),
        bottom_left: style.get_prop::<BorderBottomLeftRadius>(),
        bottom_right: style.get_prop::<BorderBottomRightRadius>(),
    }
    .debug_view()
}

fn padding_debug_view(style: &Style) -> Option<Box<dyn View>> {
    Padding {
        left: style.get_prop::<PaddingLeft>(),
        top: style.get_prop::<PaddingTop>(),
        right: style.get_prop::<PaddingRight>(),
        bottom: style.get_prop::<PaddingBottom>(),
    }
    .debug_view()
}

fn margin_debug_view(style: &Style) -> Option<Box<dyn View>> {
    Margin {
        left: style.get_prop::<MarginLeft>(),
        top: style.get_prop::<MarginTop>(),
        right: style.get_prop::<MarginRight>(),
        bottom: style.get_prop::<MarginBottom>(),
    }
    .debug_view()
}

style_debug_group!(
    pub BorderDebugGroup,
    inherited = inherited,
    members = [BorderLeft, BorderTop, BorderRight, BorderBottom],
    view = border_debug_view
);
style_debug_group!(
    pub BorderColorDebugGroup,
    inherited = inherited,
    members = [BorderLeftColor, BorderTopColor, BorderRightColor, BorderBottomColor],
    view = border_color_debug_view
);
style_debug_group!(
    pub BorderRadiusDebugGroup,
    inherited = inherited,
    members = [BorderTopLeftRadius, BorderTopRightRadius, BorderBottomLeftRadius, BorderBottomRightRadius],
    view = border_radius_debug_view
);
style_debug_group!(
    pub PaddingDebugGroup,
    inherited = inherited,
    members = [PaddingLeft, PaddingTop, PaddingRight, PaddingBottom],
    view = padding_debug_view
);
style_debug_group!(
    pub MarginDebugGroup,
    inherited = inherited,
    members = [MarginLeft, MarginTop, MarginRight, MarginBottom],
    view = margin_debug_view
);

#[derive(Debug, Clone, PartialEq)]
pub struct DesignSystem {
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub popover: Color,
    pub popover_foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
    pub sidebar: Color,
    pub sidebar_foreground: Color,
    pub sidebar_accent: Color,
    pub sidebar_accent_foreground: Color,
    pub sidebar_border: Color,
    pub sidebar_ring: Color,
    pub bg_base: Color,
    pub text_base: Color,
    pub text_lightness: f32,
    pub primary_base: Color,
    pub success_base: Color,
    pub warning_base: Color,
    pub danger_base: Color,
    pub is_dark: bool,
    pub padding: f32,
    pub border_radius: f32,
    pub font_size: f64,
}
// const BORDER_RADIUS: f32 = 5.0;
// const FONT_SIZE: f32 = 12.0;

impl DesignSystem {
    /// Create a light mode design system.
    pub fn light() -> Self {
        let background = Color::from_rgb8(255, 255, 255);
        let foreground = Color::from_rgb8(10, 10, 10);
        let primary = Color::from_rgb8(23, 23, 23);
        let secondary = Color::from_rgb8(245, 245, 245);
        let muted_foreground = Color::from_rgb8(115, 115, 115);
        let destructive = Color::from_rgb8(231, 0, 11);
        Self {
            background,
            foreground,
            card: background,
            card_foreground: foreground,
            popover: background,
            popover_foreground: foreground,
            primary,
            primary_foreground: Color::from_rgb8(250, 250, 250),
            secondary,
            secondary_foreground: primary,
            muted: secondary,
            muted_foreground,
            accent: secondary,
            accent_foreground: primary,
            destructive,
            border: Color::from_rgb8(229, 229, 229),
            input: Color::from_rgb8(229, 229, 229),
            ring: Color::from_rgb8(161, 161, 161),
            sidebar: Color::from_rgb8(250, 250, 250),
            sidebar_foreground: foreground,
            sidebar_accent: secondary,
            sidebar_accent_foreground: primary,
            sidebar_border: Color::from_rgb8(229, 229, 229),
            sidebar_ring: Color::from_rgb8(161, 161, 161),
            bg_base: background,
            text_base: foreground,
            text_lightness: 0.05,
            primary_base: primary,
            success_base: Color::from_rgb8(0x2D, 0x9D, 0x67),
            warning_base: Color::from_rgb8(0xE5, 0xA2, 0x23),
            danger_base: destructive,
            padding: 8.,
            border_radius: 10.,
            font_size: 14.,
            is_dark: false,
        }
    }

    /// Create a dark mode design system.
    pub fn dark() -> Self {
        let background = Color::from_rgb8(10, 10, 10);
        let foreground = Color::from_rgb8(250, 250, 250);
        let card = Color::from_rgb8(23, 23, 23);
        let secondary = Color::from_rgb8(38, 38, 38);
        let primary = Color::from_rgb8(229, 229, 229);
        let destructive = Color::from_rgb8(255, 100, 103);
        Self {
            background,
            foreground,
            card,
            card_foreground: foreground,
            popover: card,
            popover_foreground: foreground,
            primary,
            primary_foreground: card,
            secondary,
            secondary_foreground: foreground,
            muted: secondary,
            muted_foreground: Color::from_rgb8(161, 161, 161),
            accent: secondary,
            accent_foreground: foreground,
            destructive,
            border: Color::from_rgb8(255, 255, 255).with_alpha(0.10),
            input: Color::from_rgb8(255, 255, 255).with_alpha(0.15),
            ring: Color::from_rgb8(115, 115, 115),
            sidebar: card,
            sidebar_foreground: foreground,
            sidebar_accent: secondary,
            sidebar_accent_foreground: foreground,
            sidebar_border: Color::from_rgb8(255, 255, 255).with_alpha(0.10),
            sidebar_ring: Color::from_rgb8(115, 115, 115),
            bg_base: background,
            text_base: foreground,
            text_lightness: 0.95,
            primary_base: primary,
            success_base: Color::from_rgb8(0x4A, 0xBE, 0x8A),
            warning_base: Color::from_rgb8(0xF5, 0xB8, 0x4E),
            danger_base: destructive,
            padding: 8.,
            border_radius: 10.,
            font_size: 14.,
            is_dark: true,
        }
    }

    // Background levels

    pub fn bg_base(&self) -> Color {
        self.background
    }

    pub fn bg_elevated(&self) -> Color {
        self.muted
    }

    pub fn bg_overlay(&self) -> Color {
        self.popover
    }

    pub fn bg_disabled(&self) -> Color {
        self.muted.with_alpha(0.5)
    }

    // Border

    pub fn border(&self) -> Color {
        self.border
    }

    pub fn border_muted(&self) -> Color {
        self.border
            .with_alpha(if self.is_dark { 0.55 } else { 0.8 })
    }

    // Text

    pub fn text(&self) -> Color {
        self.foreground
    }

    pub fn text_muted(&self) -> Color {
        self.muted_foreground
    }

    // Primary

    pub fn primary(&self) -> Color {
        self.primary
    }

    pub fn primary_muted(&self) -> Color {
        self.primary.with_alpha(0.8)
    }

    pub fn primary_foreground(&self) -> Color {
        self.primary_foreground
    }

    pub fn secondary(&self) -> Color {
        self.secondary
    }

    pub fn secondary_foreground(&self) -> Color {
        self.secondary_foreground
    }

    pub fn muted(&self) -> Color {
        self.muted
    }

    pub fn muted_foreground(&self) -> Color {
        self.muted_foreground
    }

    pub fn accent(&self) -> Color {
        self.accent
    }

    pub fn accent_foreground(&self) -> Color {
        self.accent_foreground
    }

    pub fn input(&self) -> Color {
        self.input
    }

    pub fn input_muted(&self) -> Color {
        if self.is_dark {
            Color::from_rgb8(255, 255, 255).with_alpha(0.08)
        } else {
            self.input.with_alpha(0.5)
        }
    }

    pub fn input_background(&self) -> Color {
        if self.is_dark {
            Color::from_rgb8(255, 255, 255).with_alpha(0.045)
        } else {
            css::TRANSPARENT
        }
    }

    pub fn input_disabled_background(&self) -> Color {
        if self.is_dark {
            Color::from_rgb8(255, 255, 255).with_alpha(0.06)
        } else {
            self.input.with_alpha(0.5)
        }
    }

    pub fn switch_unchecked(&self) -> Color {
        if self.is_dark {
            Color::from_rgb8(255, 255, 255).with_alpha(0.12)
        } else {
            self.input
        }
    }

    pub fn ring(&self) -> Color {
        self.ring
    }

    pub fn ring_focus(&self) -> Color {
        self.ring.with_alpha(0.5)
    }

    pub fn sidebar(&self) -> Color {
        self.sidebar
    }

    pub fn sidebar_foreground(&self) -> Color {
        self.sidebar_foreground
    }

    pub fn sidebar_accent(&self) -> Color {
        self.sidebar_accent
    }

    pub fn sidebar_accent_foreground(&self) -> Color {
        self.sidebar_accent_foreground
    }

    pub fn sidebar_border(&self) -> Color {
        self.sidebar_border
    }

    pub fn sidebar_ring(&self) -> Color {
        self.sidebar_ring
    }

    pub fn button_secondary_hover(&self) -> Color {
        if self.is_dark {
            Color::from_rgb8(49, 49, 49)
        } else {
            Color::from_rgb8(233, 233, 233)
        }
    }

    // Semantic colors

    pub fn success(&self) -> Color {
        self.success_base
    }

    pub fn warning(&self) -> Color {
        self.warning_base
    }

    pub fn danger(&self) -> Color {
        self.destructive
    }

    pub fn info(&self) -> Color {
        self.primary
    }

    pub fn padding(&self) -> f32 {
        self.padding
    }

    pub fn border_radius(&self) -> f32 {
        self.border_radius
    }

    pub fn font_size(&self) -> f64 {
        self.font_size
    }
}

impl StylePropValue for DesignSystem {
    fn debug_view(&self) -> Option<AnyView> {
        use crate::prelude::*;
        use crate::views::Stack;

        let design_system = self.clone();
        let is_expanded = RwSignal::new(false);

        let color_swatch = |label: &str, color: Color| {
            Stack::new((
                label.to_string().style(|s| s.width(120.0).font_size(12.0)),
                color.debug_view().unwrap(),
            ))
            .style(|s| s.flex_row().items_center().gap(8.0).padding_vert(2.0))
        };

        let scalar_field = |label: &str, value: f64| {
            Stack::new((
                label.to_string().style(|s| s.width(120.0).font_size(12.0)),
                format!("{:.2}", value).style(|s| s.font_size(12.0)),
            ))
            .style(|s| s.flex_row().items_center().gap(8.0).padding_vert(2.0))
        };

        let chevron = move || {
            if is_expanded.get() {
                svg(
                    r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M4.427 6.427l3.396 3.396a.25.25 0 00.354 0l3.396-3.396A.25.25 0 0011.396 6H4.604a.25.25 0 00-.177.427z"/></svg>"#,
                )
            } else {
                svg(
                    r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M6.427 4.427l3.396 3.396a.25.25 0 010 .354l-3.396 3.396A.25.25 0 016 11.396V4.604a.25.25 0 01.427-.177z"/></svg>"#,
                )
            }.style(|s| s.size_full().with_theme(|s, t| s.color(t.text())))
        };

        let header = Stack::new((
            dyn_view(chevron)
                .class(ButtonClass)
                .style(|s| s.size(16.0, 16.0).padding(0.)),
            "Design System"
                .to_string()
                .style(|s| s.font_size(14.0).font_weight(FontWeight::SEMI_BOLD)),
        ))
        .action(move || {
            is_expanded.update(|v| *v = !*v);
        })
        .style(|s| {
            s.flex_row()
                .items_center()
                .gap(8.0)
                .cursor(CursorStyle::Pointer)
        });

        let content = Stack::new((
            header,
            Stack::new((
                color_swatch("background", design_system.background),
                color_swatch("foreground", design_system.foreground),
                color_swatch("primary", design_system.primary),
                color_swatch("primary_fg", design_system.primary_foreground),
                color_swatch("secondary", design_system.secondary),
                color_swatch("muted", design_system.muted),
                color_swatch("muted_fg", design_system.muted_foreground),
                color_swatch("accent", design_system.accent),
                color_swatch("destructive", design_system.destructive),
                color_swatch("border", design_system.border),
                color_swatch("sidebar", design_system.sidebar),
                color_swatch("sidebar_accent", design_system.sidebar_accent),
                scalar_field("padding", f64::from(design_system.padding)),
                scalar_field("border_radius", f64::from(design_system.border_radius)),
                scalar_field("font_size", design_system.font_size),
                format!("is_dark: {}", design_system.is_dark).style(|s| s.font_size(12.0)),
            ))
            .style(move |s| s.flex_col().gap(4.0))
            .clip()
            .style(move |s| {
                s.height_pct(100.)
                    .apply_if(!is_expanded.get(), |s| s.height_pct(0.))
                    .transition_height(Transition::ease_in_out(Duration::from_millis(200)))
            }),
        ))
        .style(|s| {
            // this view here should be getting set to have a height of just the two children combined
            // I think this is a bug in taffy
            s.flex_col()
                .padding(8.0)
                .border(1.)
                .border_color(palette::css::WHITE.with_alpha(0.3))
                .border_radius(6.0)
                .min_width(280.0)
                .min_height_pct(0.)
                .flex_grow(0.)
                .flex_shrink(1.)
        });

        Some(content.into_any())
    }

    fn interpolate(&self, other: &Self, value: f64) -> Option<Self> {
        use peniko::color::HueDirection;
        let t = value as f32;
        let inv_t = 1.0 - t;
        let t64 = value;
        let inv_t64 = 1.0 - t64;

        Some(DesignSystem {
            background: self
                .background
                .lerp(other.background, t, HueDirection::default()),
            foreground: self
                .foreground
                .lerp(other.foreground, t, HueDirection::default()),
            card: self.card.lerp(other.card, t, HueDirection::default()),
            card_foreground: self.card_foreground.lerp(
                other.card_foreground,
                t,
                HueDirection::default(),
            ),
            popover: self.popover.lerp(other.popover, t, HueDirection::default()),
            popover_foreground: self.popover_foreground.lerp(
                other.popover_foreground,
                t,
                HueDirection::default(),
            ),
            primary: self.primary.lerp(other.primary, t, HueDirection::default()),
            primary_foreground: self.primary_foreground.lerp(
                other.primary_foreground,
                t,
                HueDirection::default(),
            ),
            secondary: self
                .secondary
                .lerp(other.secondary, t, HueDirection::default()),
            secondary_foreground: self.secondary_foreground.lerp(
                other.secondary_foreground,
                t,
                HueDirection::default(),
            ),
            muted: self.muted.lerp(other.muted, t, HueDirection::default()),
            muted_foreground: self.muted_foreground.lerp(
                other.muted_foreground,
                t,
                HueDirection::default(),
            ),
            accent: self.accent.lerp(other.accent, t, HueDirection::default()),
            accent_foreground: self.accent_foreground.lerp(
                other.accent_foreground,
                t,
                HueDirection::default(),
            ),
            destructive: self
                .destructive
                .lerp(other.destructive, t, HueDirection::default()),
            border: self.border.lerp(other.border, t, HueDirection::default()),
            input: self.input.lerp(other.input, t, HueDirection::default()),
            ring: self.ring.lerp(other.ring, t, HueDirection::default()),
            sidebar: self.sidebar.lerp(other.sidebar, t, HueDirection::default()),
            sidebar_foreground: self.sidebar_foreground.lerp(
                other.sidebar_foreground,
                t,
                HueDirection::default(),
            ),
            sidebar_accent: self.sidebar_accent.lerp(
                other.sidebar_accent,
                t,
                HueDirection::default(),
            ),
            sidebar_accent_foreground: self.sidebar_accent_foreground.lerp(
                other.sidebar_accent_foreground,
                t,
                HueDirection::default(),
            ),
            sidebar_border: self.sidebar_border.lerp(
                other.sidebar_border,
                t,
                HueDirection::default(),
            ),
            sidebar_ring: self
                .sidebar_ring
                .lerp(other.sidebar_ring, t, HueDirection::default()),
            bg_base: self.bg_base.lerp(other.bg_base, t, HueDirection::default()),
            text_base: self
                .text_base
                .lerp(other.text_base, t, HueDirection::default()),
            text_lightness: self.text_lightness * inv_t + other.text_lightness * t,
            primary_base: self
                .primary_base
                .lerp(other.primary_base, t, HueDirection::default()),
            success_base: self
                .success_base
                .lerp(other.success_base, t, HueDirection::default()),
            warning_base: self
                .warning_base
                .lerp(other.warning_base, t, HueDirection::default()),
            danger_base: self
                .danger_base
                .lerp(other.danger_base, t, HueDirection::default()),
            is_dark: if t < 0.5 { self.is_dark } else { other.is_dark },
            padding: self.padding * inv_t + other.padding * t,
            border_radius: self.border_radius * inv_t + other.border_radius * t,
            font_size: self.font_size * inv_t64 + other.font_size * t64,
        })
    }
}

prop!(
    pub Theme: DesignSystem { inherited } = DesignSystem::light()
);

#[derive(Clone, Copy)]
pub struct ThemeExpr(pub(crate) ContextRef<Theme>);

impl ThemeExpr {
    pub fn def<T>(self, f: impl Fn(DesignSystem) -> T + 'static) -> ContextValue<T>
    where
        T: 'static,
    {
        self.0.def(f)
    }

    pub fn bg_base(self) -> ContextValue<Color> {
        self.def(|t| t.bg_base())
    }
    pub fn background(self) -> ContextValue<Color> {
        self.def(|t| t.background)
    }
    pub fn foreground(self) -> ContextValue<Color> {
        self.def(|t| t.foreground)
    }
    pub fn card(self) -> ContextValue<Color> {
        self.def(|t| t.card)
    }
    pub fn card_foreground(self) -> ContextValue<Color> {
        self.def(|t| t.card_foreground)
    }
    pub fn popover(self) -> ContextValue<Color> {
        self.def(|t| t.popover)
    }
    pub fn popover_foreground(self) -> ContextValue<Color> {
        self.def(|t| t.popover_foreground)
    }
    pub fn bg_elevated(self) -> ContextValue<Color> {
        self.def(|t| t.bg_elevated())
    }
    pub fn bg_overlay(self) -> ContextValue<Color> {
        self.def(|t| t.bg_overlay())
    }
    pub fn bg_disabled(self) -> ContextValue<Color> {
        self.def(|t| t.bg_disabled())
    }
    pub fn border(self) -> ContextValue<Color> {
        self.def(|t| t.border())
    }
    pub fn border_muted(self) -> ContextValue<Color> {
        self.def(|t| t.border_muted())
    }
    pub fn text(self) -> ContextValue<Color> {
        self.def(|t| t.text())
    }
    pub fn text_muted(self) -> ContextValue<Color> {
        self.def(|t| t.text_muted())
    }
    pub fn primary(self) -> ContextValue<Color> {
        self.def(|t| t.primary())
    }
    pub fn primary_muted(self) -> ContextValue<Color> {
        self.def(|t| t.primary_muted())
    }
    pub fn primary_foreground(self) -> ContextValue<Color> {
        self.def(|t| t.primary_foreground())
    }
    pub fn secondary(self) -> ContextValue<Color> {
        self.def(|t| t.secondary())
    }
    pub fn secondary_foreground(self) -> ContextValue<Color> {
        self.def(|t| t.secondary_foreground())
    }
    pub fn muted(self) -> ContextValue<Color> {
        self.def(|t| t.muted())
    }
    pub fn muted_foreground(self) -> ContextValue<Color> {
        self.def(|t| t.muted_foreground())
    }
    pub fn accent(self) -> ContextValue<Color> {
        self.def(|t| t.accent())
    }
    pub fn accent_foreground(self) -> ContextValue<Color> {
        self.def(|t| t.accent_foreground())
    }
    pub fn input(self) -> ContextValue<Color> {
        self.def(|t| t.input())
    }
    pub fn input_muted(self) -> ContextValue<Color> {
        self.def(|t| t.input_muted())
    }
    pub fn input_background(self) -> ContextValue<Color> {
        self.def(|t| t.input_background())
    }
    pub fn input_disabled_background(self) -> ContextValue<Color> {
        self.def(|t| t.input_disabled_background())
    }
    pub fn switch_unchecked(self) -> ContextValue<Color> {
        self.def(|t| t.switch_unchecked())
    }
    pub fn ring(self) -> ContextValue<Color> {
        self.def(|t| t.ring())
    }
    pub fn ring_focus(self) -> ContextValue<Color> {
        self.def(|t| t.ring_focus())
    }
    pub fn sidebar(self) -> ContextValue<Color> {
        self.def(|t| t.sidebar())
    }
    pub fn sidebar_foreground(self) -> ContextValue<Color> {
        self.def(|t| t.sidebar_foreground())
    }
    pub fn sidebar_accent(self) -> ContextValue<Color> {
        self.def(|t| t.sidebar_accent())
    }
    pub fn sidebar_accent_foreground(self) -> ContextValue<Color> {
        self.def(|t| t.sidebar_accent_foreground())
    }
    pub fn sidebar_border(self) -> ContextValue<Color> {
        self.def(|t| t.sidebar_border())
    }
    pub fn sidebar_ring(self) -> ContextValue<Color> {
        self.def(|t| t.sidebar_ring())
    }
    pub fn button_secondary_hover(self) -> ContextValue<Color> {
        self.def(|t| t.button_secondary_hover())
    }
    pub fn success(self) -> ContextValue<Color> {
        self.def(|t| t.success())
    }
    pub fn warning(self) -> ContextValue<Color> {
        self.def(|t| t.warning())
    }
    pub fn danger(self) -> ContextValue<Color> {
        self.def(|t| t.danger())
    }
    pub fn info(self) -> ContextValue<Color> {
        self.def(|t| t.info())
    }
    pub fn padding(self) -> ContextValue<Length> {
        self.def(|t| t.padding().into())
    }
    pub fn border_radius(self) -> ContextValue<Length> {
        self.def(|t| t.border_radius().into())
    }
    pub fn font_size(self) -> ContextValue<f64> {
        self.def(|t| t.font_size())
    }
    pub fn is_dark(self) -> ContextValue<bool> {
        self.def(|t| t.is_dark)
    }
    pub fn warning_base(self) -> ContextValue<Color> {
        self.def(|t| t.warning_base)
    }
}

pub trait StyleThemeExt {
    fn theme(self, theme: DesignSystem) -> Self;
    fn with_theme(self, f: impl Fn(ExprStyle, ThemeExpr) -> ExprStyle + 'static) -> Self
    where
        Self: std::marker::Sized;
}

impl StyleThemeExt for Style {
    fn theme(self, theme: DesignSystem) -> Self {
        self.set(Theme, theme)
    }
    fn with_theme(self, f: impl Fn(ExprStyle, ThemeExpr) -> ExprStyle + 'static) -> Self {
        self.with::<Theme>(|s, t| f(s, ThemeExpr(t)))
    }
}

impl StyleThemeExt for ExprStyle {
    fn theme(self, theme: DesignSystem) -> Self {
        self.set(Theme, theme)
    }
    fn with_theme(self, f: impl Fn(ExprStyle, ThemeExpr) -> ExprStyle + 'static) -> Self {
        self.with::<Theme>(|s, t| f(s, ThemeExpr(t)))
    }
}

// pub fn hover_style() -> Style {
//     Style::new().hover(|s| s.with::<Theme>(|s, t| s.translate_x(t.def(|t| t.padding))))
// }
pub fn hover_style() -> Style {
    Style::new().hover(|s| s.with_theme(|s, t| s.background(t.muted())))
}

pub fn focus_style() -> Style {
    let focus_visible_applied_style = Style::new()
        .outline(3.0)
        .with_theme(|s, t| s.outline_color(t.ring_focus()).border_color(t.ring()));

    Style::new()
        .keyboard_navigable()
        .focus_visible(|_| focus_visible_applied_style.clone())
}

pub fn border_style(with_radius: bool) -> Style {
    Style::new()
        .with_theme(move |s, t| {
            s.border_color(t.border())
                .disabled(|s| s.border_color(t.border()))
                .padding(t.padding())
                .apply_if(with_radius, |s| s.border_radius(t.border_radius()))
        })
        .border(1.0)
}

pub fn item_selected_style() -> Style {
    Style::new().selected(|s| {
        s.with_theme(|s, t| {
            s.background(t.primary())
                .color(t.primary_foreground())
                .hover(|s| s.background(t.primary_muted()))
        })
        .transition_background(Transition::linear(100.millis()))
    })
}

pub fn overlay_style() -> Style {
    Style::new()
        .with_theme(|s, t| {
            let shadow_color = Color::from_rgb8(0, 0, 0);
            s.border_color(t.border())
                .border_radius(t.border_radius())
                .padding(t.padding())
                .color(t.popover_foreground())
                .background(t.popover())
                .set_context(
                    BoxShadowProp,
                    t.def(move |theme| {
                        let base_opacity = if theme.is_dark { 0.7 } else { 0.18 };
                        smallvec![
                            BoxShadow::new()
                                .color(shadow_color.with_alpha(base_opacity * 1.2))
                                .v_offset(1.)
                                .blur_radius(2.)
                                .spread(0.),
                            BoxShadow::new()
                                .color(shadow_color.with_alpha(base_opacity * 0.8))
                                .v_offset(4.)
                                .blur_radius(8.)
                                .spread(-1.),
                            BoxShadow::new()
                                .color(shadow_color.with_alpha(base_opacity * 0.5))
                                .v_offset(12.)
                                .blur_radius(24.)
                                .spread(-4.),
                        ]
                    }),
                )
        })
        .dark_mode(|s| s.border(1).border_top(2.))
}

pub(crate) fn default_theme(os_theme: winit::window::Theme) -> Style {
    let button_style = Style::new()
        .selectable(false)
        .height(32.0)
        .padding_horiz(10.0)
        .padding_vert(0.0)
        .gap(6.0)
        .border_radius(8.0)
        .corner_smoothing(0.6)
        .border(1.0)
        .border_color(Color::TRANSPARENT)
        .font_size(14.0)
        .font_weight(FontWeight::MEDIUM)
        .with_theme(|s, t| {
            s.background(t.primary())
                .color(t.primary_foreground())
                .hover(|s| s.background(t.primary_muted()))
                .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
                .active(move |s| s.translate_y(t.def(|_| 1.0)))
        })
        .transition(Background, Transition::linear(100.millis()))
        .transition(Foreground, Transition::linear(100.millis()))
        .justify_center()
        .items_center()
        .cursor(CursorStyle::Pointer)
        .apply(focus_style());

    let checkbox_style = Style::new()
        .size(16, 16)
        .padding(0.0)
        .border(1.0)
        .border_radius(4.0)
        .with_theme(|s, t| {
            s.background(t.input_background())
                .border_color(t.input())
                .color(t.primary_foreground())
                .selected(|s| {
                    s.background(t.primary())
                        .border_color(t.primary())
                        .color(t.primary_foreground())
                })
                .disabled(|s| {
                    s.set(Opacity, 0.5)
                        .set(Cursor, Some(CursorStyle::NotAllowed))
                })
        })
        .transition(Background, Transition::linear(100.millis()))
        .transition(Foreground, Transition::linear(100.millis()))
        .cursor(CursorStyle::Pointer)
        .apply(focus_style());

    let labeled_checkbox_style = Style::new()
        .padding(0.0)
        .with_theme(|s, t| {
            s.col_gap(t.def(|_| Length::Pt(8.0)))
                .color(t.foreground())
                .disabled(|s| {
                    s.set(Opacity, 0.5)
                        .set(Cursor, Some(CursorStyle::NotAllowed))
                        .class(CheckboxClass, |s| {
                            s.set(Opacity, 0.5)
                                .set(Cursor, Some(CursorStyle::NotAllowed))
                        })
                })
        })
        .cursor(CursorStyle::Pointer)
        .transition(Background, Transition::linear(100.millis()))
        .class(CheckboxClass, |s| s.focus_none())
        .selectable(false)
        .focus(|s| s.class(CheckboxClass, |s| s.apply(focus_style())))
        .apply(focus_style());

    let radio_button_style = Style::new()
        .size(16, 16)
        .padding(0.0)
        .items_center()
        .justify_center()
        .border(1.0)
        .with_theme(|s, t| {
            s.background(t.input_background())
                .border_color(t.input())
                .selected(|s| {
                    s.background(t.primary())
                        .border_color(t.primary())
                        .color(t.primary_foreground())
                })
                .disabled(|s| {
                    s.set(Opacity, 0.5)
                        .set(Cursor, Some(CursorStyle::NotAllowed))
                })
        })
        .cursor(CursorStyle::Pointer)
        .transition(Background, Transition::linear(100.millis()))
        .transition(Foreground, Transition::linear(100.millis()))
        .border_radius(100.pct())
        .flex_shrink(0.)
        .apply(focus_style());

    let radio_button_dot_style = Style::new()
        .size(8, 8)
        .border_radius(100.0)
        .with_theme(|s, t| s.background(t.primary_foreground()));

    let labeled_radio_button_style = Style::new()
        .padding(0.0)
        .with_theme(move |s, t| {
            s.col_gap(t.def(|_| Length::Pt(8.0)))
                .set(Selectable, false)
                .color(t.foreground())
                .disabled(|s| {
                    s.set(Opacity, 0.5)
                        .set(Cursor, Some(CursorStyle::NotAllowed))
                })
        })
        .cursor(CursorStyle::Pointer)
        .class(RadioButtonClass, |s| s.focus_none())
        .transition(Background, Transition::linear(100.millis()))
        .focus(|s| s.class(RadioButtonClass, |s| s.apply(focus_style())))
        .apply(focus_style());

    let toggle_button_style = Style::new()
        .width(32.0)
        .height(18.4)
        .padding(0.0)
        .border(1.0)
        .border_color(Color::TRANSPARENT)
        .with_theme(|s, t| {
            s.background(t.switch_unchecked())
                .set_context_opt(
                    Foreground,
                    t.def(|t| {
                        Some(Brush::Solid(if t.is_dark {
                            t.foreground
                        } else {
                            t.background
                        }))
                    }),
                )
                .selected(|s| {
                    s.background(t.primary())
                        .border_color(t.primary())
                        .set_context_opt(
                            Foreground,
                            t.def(|t| {
                                Some(Brush::Solid(if t.is_dark {
                                    t.primary_foreground
                                } else {
                                    t.background
                                }))
                            }),
                        )
                })
                .disabled(|s| {
                    s.set(Opacity, 0.5)
                        .set(Cursor, Some(CursorStyle::NotAllowed))
                })
        })
        .flex_shrink(0.0)
        .border_radius(100.pct())
        .corner_smoothing(0.6)
        .set(ToggleButtonCircleRad, 8.0)
        .set(ToggleButtonInset, 0.0)
        .set(ToggleButtonUncheckedInset, Some(0.0.into()))
        .set(ToggleButtonCheckedInset, Some(2.0.into()))
        .cursor(CursorStyle::Pointer)
        .transition(Background, Transition::linear(150.millis()))
        .apply(focus_style());

    let input_style = Style::new()
        .height(32.0)
        .padding_horiz(10.0)
        .padding_vert(4.0)
        .border_radius(8.0)
        .corner_smoothing(0.6)
        .border(1.0)
        .font_size(14.0)
        .with_theme(|s, t| {
            s.background(t.input_background())
                .border_color(t.input())
                .color(t.foreground())
                .set_context(
                    SelectionColor,
                    t.def(|t| Brush::Solid(t.primary_muted().with_alpha(0.5))),
                )
                .cursor_color(t.primary())
                .disabled(|s| {
                    s.background(t.input_disabled_background())
                        .set(Opacity, 0.5)
                        .unset_cursor()
                })
        })
        .transition(Background, Transition::linear(100.millis()))
        .apply(focus_style())
        .cursor(CursorStyle::Text);

    let tab_selector_style = Style::new()
        .height(26.0)
        .padding_horiz(6.0)
        .padding_vert(2.0)
        .border(1.0)
        .border_radius(6.0)
        .background(css::TRANSPARENT)
        .border_color(css::TRANSPARENT)
        .custom_style_class(|s: LabelCustomStyle| s.selectable(false))
        .with_theme(|s, t| {
            s.color(t.def(|t| t.foreground.with_alpha(if t.is_dark { 0.7 } else { 0.6 })))
                .disabled(|s| {
                    s.set(Opacity, 0.5)
                        .set(Cursor, Some(CursorStyle::NotAllowed))
                })
                .selected(|s| {
                    s.background(t.background())
                        .color(t.foreground())
                        .border_color(t.def(|t| if t.is_dark { t.input } else { css::TRANSPARENT }))
                        .set_context(
                            BoxShadowProp,
                            t.def(|_| {
                                smallvec![
                                    BoxShadow::new()
                                        .color(Color::from_rgb8(0, 0, 0).with_alpha(0.08))
                                        .v_offset(1.0)
                                        .blur_radius(2.0)
                                ]
                            }),
                        )
                })
                .hover(|s| s.color(t.foreground()))
        })
        .transition(Background, Transition::linear(100.millis()))
        .transition(Foreground, Transition::linear(100.millis()))
        .justify_center()
        .items_center()
        .text_clip()
        .selectable(false)
        .apply(focus_style());

    // let item_unfocused_style = Style::new().with_theme(|s, t| {
    //     s.hover(|s| s.background(t.bg_elevated())).selected(|s| {
    //         s.background(t.bg_elevated())
    //             .hover(|s| s.background(t.bg_overlay()))
    //     })
    // });

    Style::new()
        .debug_group(BorderDebugGroup)
        .debug_group(BorderColorDebugGroup)
        .debug_group(BorderRadiusDebugGroup)
        .debug_group(PaddingDebugGroup)
        .debug_group(MarginDebugGroup)
        .apply_if(os_theme == winit::window::Theme::Light, |s| {
            let light = DesignSystem::light();
            s.color(light.text())
                .font_size(light.font_size())
                .background(light.bg_base())
                .color(light.text())
                .theme(light)
        })
        .apply_if(os_theme == winit::window::Theme::Dark, |s| {
            let dark = DesignSystem::dark();
            s.color(dark.text())
                .font_size(dark.font_size())
                .background(dark.bg_base())
                .color(dark.text())
                .theme(dark)
        })
        .line_height(1.2)
        .class(LabelClass, |s| {
            s.with_theme(|s, t| {
                s.custom(|s: LabelCustomExprStyle| {
                    s.selection_color(t.def(|t| Brush::Solid(t.primary_muted().with_alpha(0.5))))
                })
            })
            .with::<Selectable>(|s, selectable| {
                s.set_context_opt(
                    Cursor,
                    selectable.def(|selectable| {
                        if selectable {
                            Some(CursorStyle::Text)
                        } else {
                            None
                        }
                    }),
                )
            })
            .focusable()
        })
        .class(ListClass, |s| {
            s.apply(focus_style()).class(ListItemClass, |s| {
                s.with_theme(|s, t| {
                    s.hover(|s| s.background(t.bg_elevated())).selected(|s| {
                        s.background(t.primary())
                            .color(t.bg_base())
                            .hover(|s| s.background(t.primary_muted()))
                            .transition_background(Transition::linear(100.millis()))
                    })
                })
                .with_theme(|s, t| s.border_radius(t.border_radius()).padding_left(t.padding()))
                .items_center()
            })
        })
        .class(CheckboxClass, |_| checkbox_style)
        .class(LabeledCheckboxClass, |_| labeled_checkbox_style)
        .class(RadioButtonClass, |_| radio_button_style)
        .class(RadioButtonDotClass, |_| radio_button_dot_style)
        .class(LabeledRadioButtonClass, |_| labeled_radio_button_style)
        .class(TextInputClass, |_| input_style)
        .class(ButtonClass, |_| button_style)
        .class(TabSelectorClass, |_| tab_selector_style)
        .custom_style_class(|s: scroll::ScrollCustomStyle| {
            s.handle_border_radius(4.0)
                .handle_rounded(false)
                .apply_if(cfg!(target_os = "macos"), |s| s.handle_rounded(true))
        })
        .class(scroll::Handle, |s| {
            s.with_theme(|s, t| {
                s.background(t.border())
                    .active(|s| s.background(t.text_muted()))
                    .hover(|s| s.background(t.text_muted()))
            })
            .transition_background(Transition::ease_in_out(Duration::from_millis(300)))
        })
        .class(scroll::Track, |s| {
            s.with_theme(|s, t| s.hover(|s| s.background(t.def(|t| t.border().with_alpha(0.3)))))
                .background(css::TRANSPARENT)
                .transition_background(Transition::ease_in_out(Duration::from_millis(300)))
        })
        .class(ToggleButtonClass, |_| toggle_button_style)
        .class(SliderClass, |s| {
            s.apply(focus_style())
                .custom(|cs: SliderCustomStyle| {
                    cs.bar_radius(100.pct())
                        .accent_bar_radius(100.pct())
                        .bar_height(4.0)
                        .accent_bar_height(4.0)
                        .handle_radius(6.0)
                        .edge_align(false)
                })
                .height(16.0)
                .with_theme(|s, t| {
                    s.custom(|cs: SliderCustomExprStyle| {
                        cs.bar_color(t.def(|t| Some(Brush::Solid(t.muted()))))
                            .accent_bar_color(t.def(|t| Brush::Solid(t.primary())))
                            .handle_color(t.def(|_| Some(Brush::Solid(css::WHITE))))
                    })
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
                })
        })
        .class(PlaceholderTextClass, |s| {
            s.with_theme(|s, t| {
                s.color(t.text_muted())
                    .disabled(|s| s.color(t.def(|t| t.text_muted().with_alpha(0.5))))
            })
        })
        .class(TooltipClass, |s| {
            s.padding_horiz(12.0)
                .padding_vert(6.0)
                .max_width(320.0)
                .border(0.0)
                .border_radius(6.0)
                .corner_smoothing(0.6)
                .font_size(12.0)
                .line_height(1.0)
                .selectable(false)
                .with_theme(|s, t| s.background(t.foreground()).color(t.background()))
        })
        .class(dropdown::DropdownClass, move |s| {
            s.height(32.0)
                .min_width(144.0)
                .padding_left(10.0)
                .padding_right(8.0)
                .padding_vert(0.0)
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .font_size(14.0)
                .apply(focus_style())
                .transition(Background, Transition::linear(100.millis()))
                .transition(Foreground, Transition::linear(100.millis()))
                .with_theme(|s, t| {
                    s.background(t.input_background())
                        .border_color(t.input())
                        .color(t.foreground())
                        .hover(|s| s.background(t.input_background()))
                        .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
                })
                .selectable(false)
                .class(dropdown::DropdownPreviewClass, |s| {
                    s.gap(6.0)
                        .items_center()
                        .justify_between()
                        .class(SvgClass, |s| {
                            s.with_theme(|s, t| {
                                s.color(t.muted_foreground())
                                    .hover(|s| s.color(t.muted_foreground()))
                            })
                            .padding(0.0)
                            .size(16.0, 16.0)
                            .flex_shrink(0.0)
                        })
                })
                .class(scroll::ScrollClass, move |s| {
                    s.width_full()
                        .max_height(256.0)
                        .scrollbar_width(10.0)
                        .margin_top(4.0)
                        .padding(4.0)
                        .border(1.0)
                        .border_radius(8.0)
                        .corner_smoothing(0.6)
                        .with_theme(|s, t| {
                            let shadow_color = Color::from_rgb8(0, 0, 0);
                            s.background(t.popover())
                                .color(t.popover_foreground())
                                .border_color(t.def(|t| t.foreground.with_alpha(0.10)))
                                .set_context(
                                    BoxShadowProp,
                                    t.def(move |theme| {
                                        let opacity = if theme.is_dark { 0.5 } else { 0.14 };
                                        smallvec![
                                            BoxShadow::new()
                                                .color(shadow_color.with_alpha(opacity))
                                                .v_offset(4.0)
                                                .blur_radius(10.0)
                                                .spread(-1.0),
                                            BoxShadow::new()
                                                .color(shadow_color.with_alpha(opacity * 0.7))
                                                .v_offset(2.0)
                                                .blur_radius(4.0)
                                                .spread(-2.0),
                                        ]
                                    }),
                                )
                        })
                        .items_center()
                        .class(ListItemClass, move |s| {
                            s.min_height(28.0)
                                .padding(6.0)
                                .border_radius(6.0)
                                .font_size(14.0)
                                .selectable(false)
                                .transition(Background, Transition::linear(100.millis()))
                                .transition(Foreground, Transition::linear(100.millis()))
                                .with_theme(|s, t| {
                                    s.color(t.popover_foreground())
                                        .hover(|s| {
                                            s.background(t.accent()).color(t.accent_foreground())
                                        })
                                        .selected(|s| {
                                            s.background(t.accent()).color(t.accent_foreground())
                                        })
                                        .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
                                })
                        })
                })
        })
        .class(ResizableClass, |s| s.padding_right(1))
        .class(ResizableHandleClass, |s| {
            s.custom(|cs: ResizableCustomStyle| cs.handle_thickness(1.))
                .with_theme(|s, t| {
                    s.custom(|cs: ResizableCustomExprStyle| {
                        cs.handle_color(t.def(|t| Brush::Solid(t.border())))
                            .hover(|s| s.handle_color(t.def(|t| Brush::Solid(t.ring()))))
                    })
                })
        })
        .class(HoverTargetClass, |s| {
            s.with_theme(|s, t| {
                s.padding(t.padding())
                    .border_radius(t.border_radius())
                    .background(t.bg_elevated())
                    .outline(3)
                    .file_hover(|s| s.background(t.bg_overlay()).outline_color(t.primary()))
            })
            .cursor(CursorStyle::Pointer)
            .transition(Background, Transition::linear(100.millis()))
        })
}
