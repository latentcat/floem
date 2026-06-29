//! Built-in icon libraries.
//!
//! Icon metadata is generated at compile time from SVG files under
//! `src/icons/assets`. The SVG strings are embedded with `include_str!`, so
//! lookup does not touch the filesystem at runtime.

use std::fmt;

use crate::views::{Svg, svg};

/// Built-in icon libraries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconLibrary {
    /// Lucide icons.
    Lucide,
    /// Hugeicons.
    Hugeicons,
}

impl IconLibrary {
    /// The normalized library name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lucide => "lucide",
            Self::Hugeicons => "hugeicons",
        }
    }

    fn file_prefix(self) -> &'static str {
        match self {
            Self::Lucide => "lucide-",
            Self::Hugeicons => "hugeicons-",
        }
    }
}

impl fmt::Display for IconLibrary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Embedded SVG data for an icon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconData {
    library: IconLibrary,
    name: &'static str,
    svg: &'static str,
}

impl IconData {
    /// The library this icon belongs to.
    pub fn library(self) -> IconLibrary {
        self.library
    }

    /// The normalized icon name, without the library prefix or `.svg` suffix.
    pub fn name(self) -> &'static str {
        self.name
    }

    /// Embedded SVG markup.
    pub fn svg(self) -> &'static str {
        self.svg
    }

    /// Create a Floem SVG view for this icon.
    pub fn view(self) -> Svg {
        svg(self.svg)
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_icons.rs"));

/// Return every embedded icon for a library.
pub fn all(library: IconLibrary) -> &'static [IconData] {
    generated_icons_for(library)
}

/// Return the number of embedded icons in a library.
pub fn count(library: IconLibrary) -> usize {
    all(library).len()
}

/// Look up an icon by normalized name.
///
/// The query accepts normalized names such as `search`, prefixed names such as
/// `lucide-search`, and file names such as `lucide-search.svg`.
pub fn lookup(library: IconLibrary, name: &str) -> Option<&'static IconData> {
    let normalized = normalize_query(library, name);
    generated_icon_lookup(library, normalized)
}

/// Return embedded SVG markup for an icon.
pub fn icon_svg(library: IconLibrary, name: &str) -> Option<&'static str> {
    lookup(library, name).map(|icon| icon.svg)
}

/// Create a Floem SVG view for an icon.
pub fn icon(library: IconLibrary, name: &str) -> Option<Svg> {
    lookup(library, name).map(|icon| icon.view())
}

fn normalize_query<'a>(library: IconLibrary, name: &'a str) -> &'a str {
    let name = name.strip_suffix(".svg").unwrap_or(name);
    name.strip_prefix(library.file_prefix()).unwrap_or(name)
}
