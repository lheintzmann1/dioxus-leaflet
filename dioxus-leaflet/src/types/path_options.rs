use serde::{Deserialize, Serialize};
use color::{OpaqueColor, Srgb};

pub type Color = OpaqueColor<Srgb>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineJoin {
    Arcs,
    Bevel,
    Miter,
    #[serde(rename = "miter-clip")]
    MiterClip,
    Round,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathOptions {
    /// Whether to draw stroke along the path. Set it to false to disable borders on polygons or circles.
    pub stroke: bool,

    /// Stroke color
    #[serde(with = "color_format")]
    pub color: Color,

    /// Stroke width in pixels
    pub weight: u32,

    /// Stroke opacity
    pub opacity: f32,

    /// Defines shape to be used at the end of the stroke.
    pub line_cap: LineCap,

    /// Defines shape to be used at the corners of the stroke.
    pub line_join: LineJoin,

    /// Whether to fill the path with color.
    pub fill: bool,

    /// Fill color. Defaults to the value of the color option.
    #[serde(with = "color_format")]
    pub fill_color: Color,

    /// Fill opacity.
    pub fill_opacity: f32,
}

impl Default for PathOptions {
    fn default() -> Self {
        Self {
            stroke: true,
            color: Color::from_rgb8(0x33, 0x88, 0xff),
            weight: 3,
            opacity: 1.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            fill: true,
            fill_color: Color::from_rgb8(0x33, 0x88, 0xff),
            fill_opacity: 0.2,
        }
    }
}

mod color_format {
    use super::Color;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where S : Serializer {
        let color = color.to_rgba8();
        serializer.serialize_str(&format!("rgb({r}, {g}, {b})",
            r = color.r,
            g = color.g,
            b = color.b))
    }

    pub fn deserialize<'de, D>(_: D) -> Result<Color, D::Error>
    where D : Deserializer<'de> {
        panic!("Deserializing colors not (yet) supported")
    }
}