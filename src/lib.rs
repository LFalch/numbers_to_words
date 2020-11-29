use std::iter::repeat;

pub type Digits = Vec<Digit>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Gender {
    BeforeTens,
    Masc,
    Fem,
    Neut
}

use self::Digit::*;
use self::Gender::*;

impl Digit {
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            _ => return None
        })
    }
    #[inline]
    pub fn from_byte_char(b: u8) -> Option<Self> {
        Self::from_char(b as char)
    }
    fn to_faroese_word(self, gender: Gender) -> &'static str {
        match (self, gender) {
            (Zero, _) => "null",
            (One, Neut) => "eitt",
            (One, Masc) | (One, Fem) | (One, BeforeTens) => "ein",
            (Two, Neut) | (Two, BeforeTens) => "tvey",
            (Two, Masc) => "tveir",
            (Two, Fem) => "tvær",
            (Three, Neut)  | (Three, BeforeTens) => "trý",
            (Three, Masc) => "tríggir",
            (Three, Fem) => "tríggjar",
            (Four, _) => "fýra",
            (Five, _) => "fimm",
            (Six, _) => "seks",
            (Seven, _) => "sjey",
            (Eight, _) => "átta",
            (Nine, _) => "níggju",
        }
    }
    #[inline(always)]
    fn is_zero(self) -> bool {
        if let Zero = self {
            true
        } else {
            false
        }
    }
}

pub fn str_to_rev_digits(s: &str) -> Option<Vec<Digit>> {
    let mut v = Vec::with_capacity(s.len());
    for digit in s.chars().rev().map(|c| Digit::from_char(c)) {
        if let Some(digit) = digit {
            v.push(digit);
        } else {
            return None;
        }
    }
    Some(v)
}

const TENS: &[&str] = &[
    "",
    "tíggju",
    "tjúgu",
    "trettivu",
    "fjøruti",
    "hálvtrýss",
    "trýss",
    "hálvfjerðs",
    "fýrs",
    "hálvfems",
];

pub fn hundreds_to_faroese(h: Digit, t: Digit, u: Digit, gender: Gender) -> String {
    let mut s = String::new();
    match h {
        Zero => (),
        d => {
            s.push_str(d.to_faroese_word(Neut));
            s.push_str(" hundrað ");
        }
    }

    match t {
        Zero if u == Zero && h != Zero => (),
        Zero => s.push_str(u.to_faroese_word(gender)),
        One => s.push_str(match u {
            Zero => TENS[1],
            One => "ellivu",
            Two => "tólv",
            Three => "trettan",
            Four => "fjúrtan",
            Five => "fimtan",
            Six => "sekstan",
            Seven => "seytjan",
            Eight => "átjan",
            Nine => "nítjan",
        }),
        t => {
            if let Zero = u {
            } else {
                s.push_str(u.to_faroese_word(BeforeTens));
                s.push_str("og");
            }
            s.push_str(TENS[t as u8 as usize]);
        }
    }

    s
}

const ILLIARD: &str = "illiard";
const ILLION: &str = "illión";

const MILLIONS: &[&str] = &[
    "túsund",
    "m",
    "b",
    "tr",
    "kvadr",
    "kvint",
    "sekst",
    "sept",
    "okt",
    "non",
    "des",
    "undes",
    "duodes",
    "tredes",
    "kvattuordes",
    "kvindes",
    "seksdes",
    "novemdes",
    "vigint",
    // TODO: add prefixes un- duo- tre- kvattuor- kvin- seks- novem- as well as the tens ones in a more systematic way
];

pub fn to_faroese_words(s: &str) -> Option<String> {
    let first_non_zero = s.find(|c| c != '0').unwrap_or(s.len()).min(s.len().saturating_sub(1));

    let mut digits = str_to_rev_digits(&s[first_non_zero..])?;
    let missing_digits = 3 - digits.len() % 3;

    if missing_digits != 3 {
        digits.extend(repeat(Zero).take(missing_digits));
    }

    let mut s = String::new();

    for (i, chunk) in digits.chunks(3).enumerate().rev() {
        let g = match i {
            0 | 1 => Neut,
            _ => Fem,
        };

        match chunk {
            // reverse them because we're reading in reverse
            &[u, t, h] => {
                let zero = u.is_zero() && t.is_zero() && h.is_zero();

                if !zero || (s.is_empty() && i == 0) {
                    s.push_str(&hundreds_to_faroese(h, t, u, g));

                    let illi = [ILLION, ILLIARD][i & 1];
                    if i > 0 {
                        s.push(' ');
                        let half_i = i / 2;
                        let prefix = MILLIONS[half_i];

                        s.push_str(prefix);
                        if half_i != 0 {
                            s.push_str(illi);
                            if (h, t, u) != (Zero, Zero, One) {
                                s.push_str("ir");
                            }
                        }
                        s.push(' ');
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    Some(s)
}