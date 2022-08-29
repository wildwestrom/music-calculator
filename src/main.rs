use std::fmt::Display;

use rust_music_theory::{
    chord::{Chord, Number, Quality},
    interval::Interval,
    note::PitchClass,
};

struct DisplayableChord(Chord);

impl Display for DisplayableChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.0.root, self.0.quality))
    }
}

fn chord_string(chord: &Chord) -> String {
    format!("{}-{}", chord.root, chord.quality)
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
        let offset = 12 - scale_degree.position.semitone_count;
        let root_pitch = PitchClass::from_interval(pitch, Interval::from_semitone((offset + 0) % 12).unwrap());
        let two_pitch  = PitchClass::from_interval(pitch, Interval::from_semitone((offset + 2) % 12).unwrap());
        let five_pitch = PitchClass::from_interval(pitch, Interval::from_semitone((offset + 7) % 12).unwrap());
        Self {
            root_chord: Chord::new(root_pitch, Quality::Major, Number::Triad),
            two_chord: Chord::new(two_pitch, Quality::Minor, Number::Triad),
            five_chord: Chord::new(five_pitch, Quality::Major, Number::Triad),
        }
    }
}

impl Display for TwoFiveProgression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root_chord_str = chord_string(&self.root_chord);
        let two_chord_str = chord_string(&self.two_chord);
        let five_chord_str = chord_string(&self.five_chord);
        f.write_fmt(format_args!(
            "{} {} {}",
            two_chord_str, five_chord_str, root_chord_str
        ))
    }
}

struct ScaleDegree {
    pitch: PitchClass,
    position: Interval,
}

fn main() {
    let root = ScaleDegree {
        pitch: PitchClass::from_str("C").unwrap(),
        position: Interval::from_semitone(0).unwrap(),
    };
    println!("ii-V-I");
    dbg!(&root.pitch);
    println!("{}", TwoFiveProgression::new(root));
    let two = ScaleDegree {
        pitch: PitchClass::from_str("D").unwrap(),
        position: Interval::from_semitone(2).unwrap(),
    };
    dbg!(&two.pitch);
    println!("{}", TwoFiveProgression::new(two));
    let five = ScaleDegree {
        pitch: PitchClass::from_str("G").unwrap(),
        position: Interval::from_semitone(7).unwrap(),
    };
    dbg!(&five.pitch);
    println!("{}", TwoFiveProgression::new(five));
}
