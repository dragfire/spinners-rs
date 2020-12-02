/// Generated by build script: build.rs
/// Do not modify.

pub enum SpinnerType {
    Aesthetic,
    Arc,
    Arrow,
    Arrow2,
    Arrow3,
    Balloon,
    Balloon2,
    BetaWave,
    Bounce,
    BouncingBall,
    BouncingBar,
    BoxBounce,
    BoxBounce2,
    Christmas,
    Circle,
    CircleHalves,
    CircleQuarters,
    Clock,
    Dots,
    Dots10,
    Dots11,
    Dots12,
    Dots2,
    Dots3,
    Dots4,
    Dots5,
    Dots6,
    Dots7,
    Dots8,
    Dots8Bit,
    Dots9,
    Dqpb,
    Earth,
    Flip,
    Grenade,
    GrowHorizontal,
    GrowVertical,
    Hamburger,
    Hearts,
    Layer,
    Line,
    Line2,
    Material,
    Monkey,
    Moon,
    Noise,
    Pipe,
    Point,
    Pong,
    Runner,
    Shark,
    SimpleDots,
    SimpleDotsScrolling,
    Smiley,
    SquareCorners,
    Squish,
    Star,
    Star2,
    Toggle,
    Toggle10,
    Toggle11,
    Toggle12,
    Toggle13,
    Toggle2,
    Toggle3,
    Toggle4,
    Toggle5,
    Toggle6,
    Toggle7,
    Toggle8,
    Toggle9,
    Triangle,
    Weather,
}
impl Spinner {
    fn new(spinner_type: SpinnerType) -> Self {
        match spinner_type {
            Aesthetic => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Arc => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Arrow => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Arrow2 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Arrow3 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Balloon => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Balloon2 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            BetaWave => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Bounce => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            BouncingBall => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            BouncingBar => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            BoxBounce => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            BoxBounce2 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Christmas => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Circle => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            CircleHalves => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            CircleQuarters => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Clock => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots10 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots11 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots12 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots2 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots3 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots4 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots5 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots6 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots7 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots8 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots8Bit => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dots9 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Dqpb => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Earth => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Flip => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Grenade => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            GrowHorizontal => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            GrowVertical => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Hamburger => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Hearts => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Layer => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Line => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Line2 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Material => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Monkey => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Moon => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Noise => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Pipe => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Point => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Pong => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Runner => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Shark => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            SimpleDots => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            SimpleDotsScrolling => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Smiley => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            SquareCorners => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Squish => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Star => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Star2 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle10 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle11 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle12 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle13 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle2 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle3 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle4 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle5 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle6 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle7 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle8 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Toggle9 => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Triangle => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
            Weather => Spinner::with_data(SpinnerData::new(80, vec![".", "..", "..."])),
        }
    }
}

use SpinnerType::*;

impl ToString for SpinnerType {
    fn to_string(&self) -> String {
        match self {
            Aesthetic => "Aesthetic".to_string(),
            Arc => "Arc".to_string(),
            Arrow => "Arrow".to_string(),
            Arrow2 => "Arrow2".to_string(),
            Arrow3 => "Arrow3".to_string(),
            Balloon => "Balloon".to_string(),
            Balloon2 => "Balloon2".to_string(),
            BetaWave => "BetaWave".to_string(),
            Bounce => "Bounce".to_string(),
            BouncingBall => "BouncingBall".to_string(),
            BouncingBar => "BouncingBar".to_string(),
            BoxBounce => "BoxBounce".to_string(),
            BoxBounce2 => "BoxBounce2".to_string(),
            Christmas => "Christmas".to_string(),
            Circle => "Circle".to_string(),
            CircleHalves => "CircleHalves".to_string(),
            CircleQuarters => "CircleQuarters".to_string(),
            Clock => "Clock".to_string(),
            Dots => "Dots".to_string(),
            Dots10 => "Dots10".to_string(),
            Dots11 => "Dots11".to_string(),
            Dots12 => "Dots12".to_string(),
            Dots2 => "Dots2".to_string(),
            Dots3 => "Dots3".to_string(),
            Dots4 => "Dots4".to_string(),
            Dots5 => "Dots5".to_string(),
            Dots6 => "Dots6".to_string(),
            Dots7 => "Dots7".to_string(),
            Dots8 => "Dots8".to_string(),
            Dots8Bit => "Dots8Bit".to_string(),
            Dots9 => "Dots9".to_string(),
            Dqpb => "Dqpb".to_string(),
            Earth => "Earth".to_string(),
            Flip => "Flip".to_string(),
            Grenade => "Grenade".to_string(),
            GrowHorizontal => "GrowHorizontal".to_string(),
            GrowVertical => "GrowVertical".to_string(),
            Hamburger => "Hamburger".to_string(),
            Hearts => "Hearts".to_string(),
            Layer => "Layer".to_string(),
            Line => "Line".to_string(),
            Line2 => "Line2".to_string(),
            Material => "Material".to_string(),
            Monkey => "Monkey".to_string(),
            Moon => "Moon".to_string(),
            Noise => "Noise".to_string(),
            Pipe => "Pipe".to_string(),
            Point => "Point".to_string(),
            Pong => "Pong".to_string(),
            Runner => "Runner".to_string(),
            Shark => "Shark".to_string(),
            SimpleDots => "SimpleDots".to_string(),
            SimpleDotsScrolling => "SimpleDotsScrolling".to_string(),
            Smiley => "Smiley".to_string(),
            SquareCorners => "SquareCorners".to_string(),
            Squish => "Squish".to_string(),
            Star => "Star".to_string(),
            Star2 => "Star2".to_string(),
            Toggle => "Toggle".to_string(),
            Toggle10 => "Toggle10".to_string(),
            Toggle11 => "Toggle11".to_string(),
            Toggle12 => "Toggle12".to_string(),
            Toggle13 => "Toggle13".to_string(),
            Toggle2 => "Toggle2".to_string(),
            Toggle3 => "Toggle3".to_string(),
            Toggle4 => "Toggle4".to_string(),
            Toggle5 => "Toggle5".to_string(),
            Toggle6 => "Toggle6".to_string(),
            Toggle7 => "Toggle7".to_string(),
            Toggle8 => "Toggle8".to_string(),
            Toggle9 => "Toggle9".to_string(),
            Triangle => "Triangle".to_string(),
            Weather => "Weather".to_string(),
        }
    }
}