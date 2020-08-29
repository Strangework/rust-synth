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
#[derive(Copy, Clone, PartialEq)]
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
}

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

