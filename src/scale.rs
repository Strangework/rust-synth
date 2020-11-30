use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ScaleError {
    message: String
}

impl ScaleError {
    fn new(msg: &str) -> ScaleError {
        ScaleError {message: msg.to_string()}
    }
}

impl fmt::Display for ScaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ScaleError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum LetterNote {
    C  = 0,
    CS = 1,
    D  = 2,
    DS = 3,
    E  = 4,
    F  = 5,
    FS = 6,
    G  = 7,
    GS = 8,
    A  = 9,
    AS = 10,
    B  = 11,
}

impl LetterNote {
    pub fn from_str(letter: &str) -> Result<LetterNote, ScaleError> {
        match letter {
            "C" => Ok(LetterNote::C),
            "CS" => Ok(LetterNote::CS),
            "D" => Ok(LetterNote::D),
            "DS" => Ok(LetterNote::DS),
            "E" => Ok(LetterNote::E),
            "F" => Ok(LetterNote::F),
            "FS" => Ok(LetterNote::FS),
            "G" => Ok(LetterNote::G),
            "GS" => Ok(LetterNote::GS),
            "A" => Ok(LetterNote::A),
            "AS" => Ok(LetterNote::AS),
            "B" => Ok(LetterNote::B),
            _ => Err(ScaleError::new(format!("Invalid note letter: {}", letter).as_str())), 
        }
    }

    pub fn from_num(num: i32) -> Result<LetterNote, ScaleError> {
        match num {
           0 => Ok(LetterNote::C), 
           1 => Ok(LetterNote::CS), 
           2 => Ok(LetterNote::D), 
           3 => Ok(LetterNote::DS), 
           4 => Ok(LetterNote::E), 
           5 => Ok(LetterNote::F), 
           6 => Ok(LetterNote::FS), 
           7 => Ok(LetterNote::G), 
           8 => Ok(LetterNote::GS), 
           9 => Ok(LetterNote::A), 
           10 => Ok(LetterNote::AS), 
           11 => Ok(LetterNote::B),
           _ => Err(ScaleError::new(format!("Invalid note number: {}", num).as_str())), 
        }
    }
}

impl fmt::Display for LetterNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter_char = match self {
            LetterNote::C => "C",
            LetterNote::CS => "C#",
            LetterNote::D => "D",
            LetterNote::DS => "D#",
            LetterNote::E => "E",
            LetterNote::F => "F",
            LetterNote::FS => "F#",
            LetterNote::G => "G",
            LetterNote::GS => "G#",
            LetterNote::A => "A",
            LetterNote::AS => "A#",
            LetterNote::B => "B",
        };
        write!(f, "{}", letter_char)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Note {
    pub letter: LetterNote,
    pub octave: i32,
}

impl Note {
    pub fn get_index(&self) -> i32 {
        self.octave*12 + self.letter as i32
    }

    pub fn from_str(note_str: &str) -> Result<Note, ScaleError> {
        match note_str.len() {
            2 => Ok(Note{
                    letter: LetterNote::from_str(&note_str[0..1]).unwrap(),
                    octave: note_str[1..2].parse::<i32>().unwrap(),
                }),
            3 => Ok(Note{
                    letter: LetterNote::from_str(&note_str[0..2]).unwrap(),
                    octave: note_str[2..3].parse::<i32>().unwrap(),
                }),
            _ => Err(ScaleError::new(format!("Invalid note: {}", note_str).as_str())),
        }
    }

    pub fn from_midi_num(midi_num: i32) -> Result<Note, ScaleError> {
        /*
        if (midi_num > 108 || midi_num < 21) {
            return Err(ScaleError::new(format!("{} is outside of MIDI note range (21-108)", midi_num).as_str()))
        }
        */
        let delta_from_c4 = midi_num - 60;
        // % operator technically implements remainder, this expression implements mathematical modulo
        let letter_note = ((delta_from_c4 % 12) + 12) % 12; 
        let octave = (4.0+delta_from_c4 as f32 / 12.0).floor() as i32;
        Ok(Note{
            letter: LetterNote::from_num(letter_note).unwrap(),
            octave: octave,
        })
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.letter, self.octave)
    }
}

pub trait Scale {
    fn get_frequency(&self, note: Note) -> f64;
}

pub struct EqualTemperament {
    pub ref_note: Note,
    pub ref_freq: f64,
    pub delta_ratio: f64,
}

impl Scale for EqualTemperament {
    fn get_frequency(&self, note: Note) -> f64 {
    self.ref_freq *
        self.delta_ratio.powi(
            note.get_index()-self.ref_note.get_index()
        )
    }
}

