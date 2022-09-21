use std::fmt::Display;

use anyhow::Result;
use clap::Parser;
use rust_music_theory::{
    chord::{Chord, Number, Quality},
    interval::Interval,
    note::PitchClass,
};

struct DisplayableChord<'a>(pub &'a Chord);

impl<'a> Display for DisplayableChord<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.0.root, self.0.quality))
    }
}

#[derive(Debug)]
struct TwoFiveProgression {
    root_chord: Chord,
    two_chord: Chord,
    five_chord: Chord,
}

impl TwoFiveProgression {
    fn new(scale_degree: ScaleDegree) -> Self {
        let pitch = scale_degree.pitch;
        let offset = 12 - scale_degree.interval_from_root.semitone_count;
        let root_pitch =
            PitchClass::from_interval(pitch, Interval::from_semitone((offset + 0) % 12).unwrap());
        let two_pitch =
            PitchClass::from_interval(pitch, Interval::from_semitone((offset + 2) % 12).unwrap());
        let five_pitch =
            PitchClass::from_interval(pitch, Interval::from_semitone((offset + 7) % 12).unwrap());
        Self {
            root_chord: Chord::new(root_pitch, Quality::Major, Number::Triad),
            two_chord: Chord::new(two_pitch, Quality::Minor, Number::Triad),
            five_chord: Chord::new(five_pitch, Quality::Major, Number::Triad),
        }
    }
}

impl Display for TwoFiveProgression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            DisplayableChord(&self.two_chord),
            DisplayableChord(&self.five_chord),
            DisplayableChord(&self.root_chord)
        ))
    }
}

#[derive(Debug)]
struct ScaleDegree {
    pitch: PitchClass,
    interval_from_root: Interval,
}

fn parse_interval(semitone: &str) -> Result<Interval> {
    let semitone_num = semitone.parse::<u8>()?;
    Ok(Interval::from_semitone(semitone_num)?)
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A string representing a pitch class
    pitch: PitchClass,
    /// A string representing the interval from the root note
    #[clap(value_parser = clap::builder::ValueParser::new(parse_interval))]
    interval: Interval,
}

fn main() {
    let args = Args::parse();

    let pitch = args.pitch;
    let interval_from_root = args.interval;

    let scaledeg = ScaleDegree {
        pitch,
        interval_from_root,
    };
    println!(
        "Given that {} is {} semitones from the root",
        scaledeg.pitch, scaledeg.interval_from_root.semitone_count
    );
    println!("{}", TwoFiveProgression::new(scaledeg));
}
