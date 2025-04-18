// Copyright 2019 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This file is autogenerated. Do not edit it!
// See ./codegen for details.

/// An element ID.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub enum EId {
    A,
    Circle,
    ClipPath,
    Defs,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeDropShadow,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    G,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Tref,
    Tspan,
    Use
}

static ELEMENTS: Map<EId> = Map {
    key: 732231254413039614,
    disps: &[
        (0, 12),
        (1, 11),
        (10, 26),
        (2, 42),
        (1, 19),
        (0, 5),
        (1, 13),
        (8, 50),
        (0, 0),
        (1, 0),
        (7, 45),
    ],
    entries: &[
        ("feFlood", EId::FeFlood),
        ("radialGradient", EId::RadialGradient),
        ("feImage", EId::FeImage),
        ("stop", EId::Stop),
        ("fePointLight", EId::FePointLight),
        ("feConvolveMatrix", EId::FeConvolveMatrix),
        ("feComposite", EId::FeComposite),
        ("clipPath", EId::ClipPath),
        ("feMerge", EId::FeMerge),
        ("defs", EId::Defs),
        ("mask", EId::Mask),
        ("svg", EId::Svg),
        ("symbol", EId::Symbol),
        ("linearGradient", EId::LinearGradient),
        ("feSpecularLighting", EId::FeSpecularLighting),
        ("feFuncB", EId::FeFuncB),
        ("filter", EId::Filter),
        ("feFuncG", EId::FeFuncG),
        ("circle", EId::Circle),
        ("g", EId::G),
        ("tref", EId::Tref),
        ("feFuncA", EId::FeFuncA),
        ("image", EId::Image),
        ("text", EId::Text),
        ("line", EId::Line),
        ("pattern", EId::Pattern),
        ("use", EId::Use),
        ("feDropShadow", EId::FeDropShadow),
        ("feSpotLight", EId::FeSpotLight),
        ("marker", EId::Marker),
        ("style", EId::Style),
        ("switch", EId::Switch),
        ("tspan", EId::Tspan),
        ("feColorMatrix", EId::FeColorMatrix),
        ("feOffset", EId::FeOffset),
        ("path", EId::Path),
        ("feGaussianBlur", EId::FeGaussianBlur),
        ("feTile", EId::FeTile),
        ("feTurbulence", EId::FeTurbulence),
        ("feMergeNode", EId::FeMergeNode),
        ("feMorphology", EId::FeMorphology),
        ("a", EId::A),
        ("textPath", EId::TextPath),
        ("ellipse", EId::Ellipse),
        ("feComponentTransfer", EId::FeComponentTransfer),
        ("feDistantLight", EId::FeDistantLight),
        ("polyline", EId::Polyline),
        ("polygon", EId::Polygon),
        ("feBlend", EId::FeBlend),
        ("feDisplacementMap", EId::FeDisplacementMap),
        ("feDiffuseLighting", EId::FeDiffuseLighting),
        ("rect", EId::Rect),
        ("feFuncR", EId::FeFuncR),
    ],
};

impl EId {
    pub(crate) fn from_str(text: &str) -> Option<EId> {
        ELEMENTS.get(text).cloned()
    }

    /// Returns the original string.
    #[inline(never)]
    pub fn to_str(self) -> &'static str {
        ELEMENTS.key(&self)
    }
}

impl std::fmt::Debug for EId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::fmt::Display for EId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// An attribute ID.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub enum AId {
    AlignmentBaseline,
    Amplitude,
    Azimuth,
    BackgroundColor,
    BaseFrequency,
    BaselineShift,
    Bias,
    Class,
    Clip,
    ClipPath,
    ClipRule,
    ClipPathUnits,
    Color,
    ColorInterpolation,
    ColorInterpolationFilters,
    ColorProfile,
    ColorRendering,
    Cx,
    Cy,
    D,
    DiffuseConstant,
    Direction,
    Display,
    Divisor,
    DominantBaseline,
    Dx,
    Dy,
    EdgeMode,
    Elevation,
    EnableBackground,
    Exponent,
    Fill,
    FillOpacity,
    FillRule,
    Filter,
    FilterUnits,
    FloodColor,
    FloodOpacity,
    Font,
    FontFamily,
    FontFeatureSettings,
    FontKerning,
    FontSize,
    FontSizeAdjust,
    FontStretch,
    FontStyle,
    FontSynthesis,
    FontVariant,
    FontVariantCaps,
    FontVariantEastAsian,
    FontVariantLigatures,
    FontVariantNumeric,
    FontVariantPosition,
    FontWeight,
    Fr,
    Fx,
    Fy,
    GlyphOrientationHorizontal,
    GlyphOrientationVertical,
    GradientTransform,
    GradientUnits,
    Height,
    Href,
    Id,
    ImageRendering,
    In,
    In2,
    InlineSize,
    Intercept,
    Isolation,
    K1,
    K2,
    K3,
    K4,
    KernelMatrix,
    KernelUnitLength,
    Kerning,
    LengthAdjust,
    LetterSpacing,
    LightingColor,
    LimitingConeAngle,
    LineHeight,
    MarkerEnd,
    MarkerMid,
    MarkerStart,
    MarkerHeight,
    MarkerUnits,
    MarkerWidth,
    Mask,
    MaskBorder,
    MaskBorderMode,
    MaskBorderOutset,
    MaskBorderRepeat,
    MaskBorderSlice,
    MaskBorderSource,
    MaskBorderWidth,
    MaskClip,
    MaskComposite,
    MaskImage,
    MaskMode,
    MaskOrigin,
    MaskPosition,
    MaskSize,
    MaskType,
    MaskContentUnits,
    MaskUnits,
    MixBlendMode,
    Mode,
    NumOctaves,
    Offset,
    Opacity,
    Operator,
    Order,
    Orient,
    Overflow,
    PaintOrder,
    Path,
    PathLength,
    PatternContentUnits,
    PatternTransform,
    PatternUnits,
    Points,
    PointsAtX,
    PointsAtY,
    PointsAtZ,
    PreserveAlpha,
    PreserveAspectRatio,
    PrimitiveUnits,
    R,
    Radius,
    RefX,
    RefY,
    RequiredExtensions,
    RequiredFeatures,
    Result,
    Rotate,
    Rx,
    Ry,
    Scale,
    Seed,
    ShapeImageThreshold,
    ShapeInside,
    ShapeMargin,
    ShapePadding,
    ShapeRendering,
    ShapeSubtract,
    Side,
    Slope,
    Space,
    SpecularConstant,
    SpecularExponent,
    SpreadMethod,
    StartOffset,
    StdDeviation,
    StitchTiles,
    StopColor,
    StopOpacity,
    Stroke,
    StrokeDasharray,
    StrokeDashoffset,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeOpacity,
    StrokeWidth,
    Style,
    SurfaceScale,
    SystemLanguage,
    TableValues,
    TargetX,
    TargetY,
    TextAlign,
    TextAlignLast,
    TextAnchor,
    TextDecoration,
    TextDecorationColor,
    TextDecorationFill,
    TextDecorationLine,
    TextDecorationStroke,
    TextDecorationStyle,
    TextIndent,
    TextOrientation,
    TextOverflow,
    TextRendering,
    TextUnderlinePosition,
    TextLength,
    Transform,
    TransformBox,
    TransformOrigin,
    Type,
    UnicodeBidi,
    UnicodeRange,
    Values,
    VectorEffect,
    ViewBox,
    Visibility,
    WhiteSpace,
    Width,
    WordSpacing,
    WritingMode,
    X,
    X1,
    X2,
    XChannelSelector,
    Y,
    Y1,
    Y2,
    YChannelSelector,
    Z
}

static ATTRIBUTES: Map<AId> = Map {
    key: 3347381344252206323,
    disps: &[
        (0, 111),
        (0, 2),
        (0, 45),
        (0, 5),
        (0, 1),
        (2, 56),
        (0, 5),
        (2, 99),
        (13, 198),
        (0, 61),
        (0, 52),
        (1, 29),
        (0, 21),
        (0, 70),
        (0, 164),
        (2, 60),
        (3, 52),
        (0, 1),
        (0, 86),
        (0, 10),
        (0, 0),
        (0, 4),
        (2, 175),
        (6, 59),
        (1, 14),
        (0, 13),
        (3, 175),
        (1, 10),
        (2, 76),
        (0, 53),
        (0, 24),
        (123, 202),
        (0, 14),
        (0, 30),
        (0, 62),
        (0, 98),
        (11, 193),
        (8, 79),
        (0, 17),
        (22, 5),
        (36, 106),
        (1, 1),
    ],
    entries: &[
        ("mask-border-source", AId::MaskBorderSource),
        ("stop-opacity", AId::StopOpacity),
        ("stroke-linejoin", AId::StrokeLinejoin),
        ("dominant-baseline", AId::DominantBaseline),
        ("spreadMethod", AId::SpreadMethod),
        ("order", AId::Order),
        ("stroke", AId::Stroke),
        ("stitchTiles", AId::StitchTiles),
        ("height", AId::Height),
        ("font-size", AId::FontSize),
        ("background-color", AId::BackgroundColor),
        ("tableValues", AId::TableValues),
        ("x1", AId::X1),
        ("y", AId::Y),
        ("width", AId::Width),
        ("text-indent", AId::TextIndent),
        ("fill-opacity", AId::FillOpacity),
        ("word-spacing", AId::WordSpacing),
        ("cy", AId::Cy),
        ("scale", AId::Scale),
        ("x2", AId::X2),
        ("lengthAdjust", AId::LengthAdjust),
        ("glyph-orientation-horizontal", AId::GlyphOrientationHorizontal),
        ("opacity", AId::Opacity),
        ("mask-border", AId::MaskBorder),
        ("font-stretch", AId::FontStretch),
        ("stroke-dashoffset", AId::StrokeDashoffset),
        ("fill", AId::Fill),
        ("space", AId::Space),
        ("baseline-shift", AId::BaselineShift),
        ("text-align-last", AId::TextAlignLast),
        ("font-variant-east-asian", AId::FontVariantEastAsian),
        ("mask-border-mode", AId::MaskBorderMode),
        ("font-variant-caps", AId::FontVariantCaps),
        ("gradientUnits", AId::GradientUnits),
        ("exponent", AId::Exponent),
        ("text-decoration-color", AId::TextDecorationColor),
        ("refX", AId::RefX),
        ("enable-background", AId::EnableBackground),
        ("mask-border-width", AId::MaskBorderWidth),
        ("numOctaves", AId::NumOctaves),
        ("kerning", AId::Kerning),
        ("mix-blend-mode", AId::MixBlendMode),
        ("mask-clip", AId::MaskClip),
        ("mask-mode", AId::MaskMode),
        ("type", AId::Type),
        ("class", AId::Class),
        ("font", AId::Font),
        ("mask-border-repeat", AId::MaskBorderRepeat),
        ("stroke-miterlimit", AId::StrokeMiterlimit),
        ("text-decoration-stroke", AId::TextDecorationStroke),
        ("z", AId::Z),
        ("dx", AId::Dx),
        ("clip-path", AId::ClipPath),
        ("markerHeight", AId::MarkerHeight),
        ("text-underline-position", AId::TextUnderlinePosition),
        ("stdDeviation", AId::StdDeviation),
        ("id", AId::Id),
        ("paint-order", AId::PaintOrder),
        ("elevation", AId::Elevation),
        ("specularConstant", AId::SpecularConstant),
        ("result", AId::Result),
        ("font-size-adjust", AId::FontSizeAdjust),
        ("mask-origin", AId::MaskOrigin),
        ("direction", AId::Direction),
        ("font-variant-numeric", AId::FontVariantNumeric),
        ("startOffset", AId::StartOffset),
        ("maskUnits", AId::MaskUnits),
        ("font-variant", AId::FontVariant),
        ("text-orientation", AId::TextOrientation),
        ("amplitude", AId::Amplitude),
        ("rx", AId::Rx),
        ("mask-type", AId::MaskType),
        ("filter", AId::Filter),
        ("in", AId::In),
        ("display", AId::Display),
        ("seed", AId::Seed),
        ("unicode-range", AId::UnicodeRange),
        ("color-profile", AId::ColorProfile),
        ("x", AId::X),
        ("href", AId::Href),
        ("font-feature-settings", AId::FontFeatureSettings),
        ("fill-rule", AId::FillRule),
        ("fr", AId::Fr),
        ("font-variant-ligatures", AId::FontVariantLigatures),
        ("text-decoration-style", AId::TextDecorationStyle),
        ("radius", AId::Radius),
        ("xChannelSelector", AId::XChannelSelector),
        ("orient", AId::Orient),
        ("isolation", AId::Isolation),
        ("gradientTransform", AId::GradientTransform),
        ("transform-box", AId::TransformBox),
        ("pointsAtY", AId::PointsAtY),
        ("text-decoration-line", AId::TextDecorationLine),
        ("requiredFeatures", AId::RequiredFeatures),
        ("patternContentUnits", AId::PatternContentUnits),
        ("shape-padding", AId::ShapePadding),
        ("text-overflow", AId::TextOverflow),
        ("clipPathUnits", AId::ClipPathUnits),
        ("azimuth", AId::Azimuth),
        ("line-height", AId::LineHeight),
        ("viewBox", AId::ViewBox),
        ("preserveAspectRatio", AId::PreserveAspectRatio),
        ("path", AId::Path),
        ("k4", AId::K4),
        ("systemLanguage", AId::SystemLanguage),
        ("stroke-width", AId::StrokeWidth),
        ("specularExponent", AId::SpecularExponent),
        ("writing-mode", AId::WritingMode),
        ("transform-origin", AId::TransformOrigin),
        ("stroke-linecap", AId::StrokeLinecap),
        ("points", AId::Points),
        ("style", AId::Style),
        ("pointsAtZ", AId::PointsAtZ),
        ("targetX", AId::TargetX),
        ("font-synthesis", AId::FontSynthesis),
        ("maskContentUnits", AId::MaskContentUnits),
        ("text-align", AId::TextAlign),
        ("cx", AId::Cx),
        ("alignment-baseline", AId::AlignmentBaseline),
        ("font-kerning", AId::FontKerning),
        ("requiredExtensions", AId::RequiredExtensions),
        ("clip-rule", AId::ClipRule),
        ("mask-border-outset", AId::MaskBorderOutset),
        ("primitiveUnits", AId::PrimitiveUnits),
        ("textLength", AId::TextLength),
        ("text-decoration-fill", AId::TextDecorationFill),
        ("fy", AId::Fy),
        ("mask-size", AId::MaskSize),
        ("k3", AId::K3),
        ("marker-start", AId::MarkerStart),
        ("mode", AId::Mode),
        ("k1", AId::K1),
        ("refY", AId::RefY),
        ("y1", AId::Y1),
        ("shape-rendering", AId::ShapeRendering),
        ("operator", AId::Operator),
        ("mask-image", AId::MaskImage),
        ("marker-end", AId::MarkerEnd),
        ("rotate", AId::Rotate),
        ("limitingConeAngle", AId::LimitingConeAngle),
        ("surfaceScale", AId::SurfaceScale),
        ("intercept", AId::Intercept),
        ("font-variant-position", AId::FontVariantPosition),
        ("clip", AId::Clip),
        ("fx", AId::Fx),
        ("visibility", AId::Visibility),
        ("shape-margin", AId::ShapeMargin),
        ("font-style", AId::FontStyle),
        ("y2", AId::Y2),
        ("dy", AId::Dy),
        ("yChannelSelector", AId::YChannelSelector),
        ("ry", AId::Ry),
        ("color-rendering", AId::ColorRendering),
        ("white-space", AId::WhiteSpace),
        ("patternUnits", AId::PatternUnits),
        ("shape-subtract", AId::ShapeSubtract),
        ("markerWidth", AId::MarkerWidth),
        ("d", AId::D),
        ("shape-inside", AId::ShapeInside),
        ("preserveAlpha", AId::PreserveAlpha),
        ("shape-image-threshold", AId::ShapeImageThreshold),
        ("image-rendering", AId::ImageRendering),
        ("marker-mid", AId::MarkerMid),
        ("filterUnits", AId::FilterUnits),
        ("bias", AId::Bias),
        ("mask-border-slice", AId::MaskBorderSlice),
        ("pointsAtX", AId::PointsAtX),
        ("kernelMatrix", AId::KernelMatrix),
        ("color-interpolation", AId::ColorInterpolation),
        ("glyph-orientation-vertical", AId::GlyphOrientationVertical),
        ("color", AId::Color),
        ("patternTransform", AId::PatternTransform),
        ("kernelUnitLength", AId::KernelUnitLength),
        ("markerUnits", AId::MarkerUnits),
        ("font-weight", AId::FontWeight),
        ("overflow", AId::Overflow),
        ("stop-color", AId::StopColor),
        ("r", AId::R),
        ("k2", AId::K2),
        ("text-anchor", AId::TextAnchor),
        ("inline-size", AId::InlineSize),
        ("unicode-bidi", AId::UnicodeBidi),
        ("font-family", AId::FontFamily),
        ("color-interpolation-filters", AId::ColorInterpolationFilters),
        ("slope", AId::Slope),
        ("baseFrequency", AId::BaseFrequency),
        ("transform", AId::Transform),
        ("text-rendering", AId::TextRendering),
        ("divisor", AId::Divisor),
        ("edgeMode", AId::EdgeMode),
        ("letter-spacing", AId::LetterSpacing),
        ("flood-color", AId::FloodColor),
        ("in2", AId::In2),
        ("side", AId::Side),
        ("mask-composite", AId::MaskComposite),
        ("offset", AId::Offset),
        ("values", AId::Values),
        ("vector-effect", AId::VectorEffect),
        ("mask", AId::Mask),
        ("pathLength", AId::PathLength),
        ("lighting-color", AId::LightingColor),
        ("mask-position", AId::MaskPosition),
        ("stroke-dasharray", AId::StrokeDasharray),
        ("text-decoration", AId::TextDecoration),
        ("stroke-opacity", AId::StrokeOpacity),
        ("targetY", AId::TargetY),
        ("flood-opacity", AId::FloodOpacity),
        ("diffuseConstant", AId::DiffuseConstant),
    ],
};

impl AId {
    pub(crate) fn from_str(text: &str) -> Option<AId> {
        ATTRIBUTES.get(text).cloned()
    }

    /// Returns the original string.
    #[inline(never)]
    pub fn to_str(self) -> &'static str {
        ATTRIBUTES.key(&self)
    }
}

impl std::fmt::Debug for AId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::fmt::Display for AId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// A stripped down `phf` crate fork.
//
// https://github.com/sfackler/rust-phf

struct Map<V: 'static> {
    pub key: u64,
    pub disps: &'static [(u32, u32)],
    pub entries: &'static [(&'static str, V)],
}

impl<V: PartialEq> Map<V> {
    fn get(&self, key: &str) -> Option<&V> {
        let hash = hash(key, self.key);
        let index = get_index(hash, self.disps, self.entries.len());
        let entry = &self.entries[index as usize];
        let b = entry.0;
        if b == key {
            Some(&entry.1)
        } else {
            None
        }
    }

    fn key(&self, value: &V) -> &'static str {
        self.entries.iter().find(|kv| kv.1 == *value).unwrap().0
    }
}

#[inline]
fn hash(x: &str, key: u64) -> u64 {
    use std::hash::Hasher;

    let mut hasher = siphasher::sip::SipHasher13::new_with_keys(0, key);
    hasher.write(x.as_bytes());
    hasher.finish()
}

#[inline]
fn get_index(hash: u64, disps: &[(u32, u32)], len: usize) -> u32 {
    let (g, f1, f2) = split(hash);
    let (d1, d2) = disps[(g % (disps.len() as u32)) as usize];
    displace(f1, f2, d1, d2) % (len as u32)
}

#[inline]
fn split(hash: u64) -> (u32, u32, u32) {
    const BITS: u32 = 21;
    const MASK: u64 = (1 << BITS) - 1;

    ((hash & MASK) as u32,
     ((hash >> BITS) & MASK) as u32,
     ((hash >> (2 * BITS)) & MASK) as u32)
}

#[inline]
fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    d2 + f1 * d1 + f2
}
