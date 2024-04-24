#![forbid(clippy::unwrap_used)]

macro_rules! value_enum {
    ($enum_name:ident, $($values:ident),+) => {
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum $enum_name {
            $($values),+
        }
    }
}

/**
 * The class for CSS selectors, e.g. .x#y .e5
 */
#[derive(Clone, PartialEq, Debug)]
pub struct Selector {
    pub ids: Vec<String>,
    pub classes: Vec<String>,
    pub parent: Option<Box<Selector>>,
}

impl From<&str> for Selector {
    fn from(string: &str) -> Selector {
        let (mut ids, mut classes, mut parent) = (vec![], vec![], None);
        let mut selectors = string.split_whitespace().collect::<Vec<&str>>();
        selectors.reverse();
        let mut id = selectors[0]
            .split('#')
            .skip(1)
            .map(|x| x.split('.').collect::<Vec<&str>>()[0].to_string())
            .collect::<Vec<String>>();
        ids.append(&mut id);
        let mut class = selectors[0]
            .split('.')
            .skip(1)
            .map(|x| x.split('#').collect::<Vec<&str>>()[0].to_string())
            .collect::<Vec<String>>();
        classes.append(&mut class);

        if selectors.len() > 1 {
            parent = Some(Box::from(Selector::from(selectors[1])));
        }

        Selector {
            ids,
            classes,
            parent,
        }
    }
}

// TODO: Implement "Into<String>" for "Selector"
impl From<Selector> for String {
    fn from(_val: Selector) -> Self {
        todo!()
    }
}

impl From<String> for Selector {
    fn from(x: String) -> Selector {
        Selector::from(x.as_str())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Value<T: Copy> {
    Normal(T),
    Global(GlobalValue),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimeValue {
    Milliseconds(u64),
    Seconds(u32),
    Minutes(u16),
    Hours(u8),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AnimationTimingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
    /*
     * start = true
     * end = false
     */
    Steps(i64, bool),
    CubicBezier(f64, f64, f64, f64),
}

// TODO: Add methods to convert from "hex", "hsla", "cmyk", etc.
// TODO: Add a set of default color values, related to the ones in CSS.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub alpha: f64,
}

value_enum!(
    FlexItemDirection,
    Stretch,
    Center,
    Baseline,
    FlexStart,
    FlexEnd,
    SpaceBetween,
    SpaceAround
);
value_enum!(GlobalValue, Initial, Inherit, Unset);
value_enum!(
    AnimationDirection,
    Normal,
    Reverse,
    Alternate,
    AlternateReverse
);
value_enum!(AnimationFillMode, None, Forwards, Backwards, Both);
value_enum!(BackgroundAttachment, Scroll, Fixed, Local);
value_enum!(
    BackgroundBlendMode,
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    Saturation,
    Color,
    Luminosity
);
value_enum!(BackgroundClip, ContentBox, PaddingBox, BorderBox);

#[derive(Clone, PartialEq, Debug)]
pub enum Rule {
    AlignContent(Value<FlexItemDirection>),
    AlignItems(Value<FlexItemDirection>),
    AlignSelf(Value<FlexItemDirection>),
    All(GlobalValue),
    AnimationDelay(Value<TimeValue>),
    AnimationDirection(Value<AnimationDirection>),
    AnimationDuration(Value<TimeValue>),
    AnimationFillMode(Value<AnimationFillMode>),
    AnimationIterationCount(Value<u64>),
    // Using [char; 256] instead of String to allow the Copy trait
    AnimationName(Box<Value<[char; 256]>>),
    /*
     * running = true
     * paused = false
     */
    AnimationPlayState(Value<bool>),
    AnimationTimingFunction(Value<AnimationTimingFunction>),
    /*
     * visible = true
     * hidden = false
     */
    BackfaceVisibility(Value<bool>),
    BackgroundAttachment(Value<BackgroundAttachment>),
    BackgroundBlendMode(Value<BackgroundBlendMode>),
    BackgroundClip(Value<BackgroundClip>),
    BackgroundColor(Color),
}

pub struct Ruleset {
    pub rules: Vec<Rule>,
    pub selector: Selector,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn selector() {
        assert_eq!(
            Selector::from("#abc#efg.hi .abc#de#fg"),
            Selector {
                parent: Some(Box::from(Selector {
                    ids: vec!["abc".to_string()],
                    classes: vec![],
                    parent: None,
                })),
                classes: vec!["abc".to_string()],
                ids: vec![]
            }
        );
    }
}
