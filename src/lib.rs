// @author      ::  Preston Wang-Stosur-Bassett <http://stosur.info>
// @date        ::  December 8, 2017
// @description ::  This file takes pinyin with tone numbers and returns pinyin with tone marks

//! ### About
//! Turn pinyin written with tone numbers and turn it into pinyin with node marks. prettify_pinyin
//! accepts input in the [CC-CEDICT](https://cc-cedict.org/wiki/format:syntax) pinyin format (space
//! separated syllables with tone numbers at the end of each syllable), for example: "ni3 hao3" will
//! get turned into "nǐ hǎo".
//!
//! ### Usage
//! ```rust
//! use prettify_pinyin::prettify;
//!
//! let formatted: String = prettify("ma1 ma2 ma3 ma4 ma");
//!
//! println!("{}", formatted); // --> mā má mǎ mà ma
//! ```

use std::borrow::Cow;

#[cfg(test)]
mod tests {
    use super::prettify;

    #[test]
    fn prettify_basic() {
        let hello = "nǐ hǎo";
        let china = "zhōng guó";
        let all_tones = "mā má mǎ mà";
        let no_tones = "ma";
        let capital_letter = "Ān huī";
        let yelling = "NǏ HǍO ZHŌNG GUÓ";

        assert_eq!(hello, prettify("ni3 hao3"));
        assert_eq!(china, prettify("zhong1 guo2"));
        assert_eq!(all_tones, prettify("ma1 ma2 ma3 ma4"));
        assert_eq!(no_tones, prettify("ma"));
        assert_eq!(capital_letter, prettify("An1 hui1"));
        assert_eq!(yelling, prettify("NI3 HAO3 ZHONG1 GUO2"));
    }

    #[test]
    fn prettify_umlaut() {
        assert_eq!("nǚ nǚ", prettify("nu:3 nu:3"));
        assert_eq!("NǙ", prettify("NU:3"));
        assert_eq!("nǚ NǙ", prettify("nu:3 NU:3"));
    }

    #[test]
    fn invalid_tone() {
        assert_eq!("ni7", prettify("ni7"));
    }

    #[test]
    fn clear_tones() {
        assert_eq!("ni", prettify("nǐ5"));
        assert_eq!("nǚ nü", prettify("nǚ nǚ5"));
    }

    #[test]
    fn reassign_tones() {
        assert_eq!("nī", prettify("nǐ1"));
        assert_eq!("nǘ nǜ", prettify("nǚ2 nǚ4"));
    }
}

static REPLACEMENTS: [(char, [char; 5]); 12] = [
    ('a', ['ā', 'á', 'ǎ', 'à', 'a']),
    ('e', ['ē', 'é', 'ě', 'è', 'e']),
    ('u', ['ū', 'ú', 'ǔ', 'ù', 'u']),
    ('i', ['ī', 'í', 'ǐ', 'ì', 'i']),
    ('o', ['ō', 'ó', 'ǒ', 'ò', 'o']),
    ('ü', ['ǖ', 'ǘ', 'ǚ', 'ǜ', 'ü']),
    ('A', ['Ā', 'Á', 'Ǎ', 'À', 'A']),
    ('E', ['Ē', 'É', 'Ě', 'È', 'E']),
    ('U', ['Ū', 'Ú', 'Ǔ', 'Ù', 'U']),
    ('I', ['Ī', 'Í', 'Ǐ', 'Ì', 'I']),
    ('O', ['Ō', 'Ó', 'Ǒ', 'Ò', 'O']),
    ('Ü', ['Ǖ', 'Ǘ', 'Ǚ', 'Ǜ', 'U']),
];

fn lookup_replacement(input: char, tone: u8) -> Option<char> {
    REPLACEMENTS.iter().find_map(|(key, tones)| {
        if *key == input {
            Some(tones[tone.saturating_sub(1) as usize % tones.len()])
        } else {
            None
        }
    })
}

fn clear_tone_mark(input: char) -> char {
    for (letter, tone_marks) in REPLACEMENTS.iter() {
        if tone_marks.contains(&input) {
            return *letter;
        }
    }
    input
}

/// # prettify
/// ```
/// use prettify_pinyin::prettify;
/// prettify("ma1 ma2 ma3 ma4 ma"); // --> mā má mǎ mà ma
/// ```
pub fn prettify(text: &str) -> String {
    let medials: &str = "iuüIUÜ";

    let text = text.replace('v', "ü");
    let text = text.replace('V', "Ü");
    let text = text.replace("u:", "ü");
    let text = text.replace("U:", "Ü");

    let syllables = text.split(' ').map(|syllable| {
        let mut chars: Vec<char> = syllable.chars().map(clear_tone_mark).collect();
        let tone: u8 = match chars.last().and_then(|c| c.to_digit(6)) {
            Some(tone) => tone as u8,
            None => return Cow::Borrowed(syllable),
        };

        if tone == 0_u8 || tone > 5_u8 {
            // This is not a valid number
            Cow::Borrowed(syllable)
        } else {
            for i in 0..chars.len() - 1 {
                let current_letter = chars[i];
                let next_letter = chars[i + 1];
                if let Some(new_current_letter) = lookup_replacement(current_letter, tone) {
                    if let Some(new_next_letter) = lookup_replacement(next_letter, tone) {
                        if medials.contains(current_letter) {
                            // if 'i', 'u' or 'ü' precedes a vowel, put the tone-mark over that vowel instead
                            chars[i + 1] = new_next_letter;
                            break;
                        }
                    }
                    chars[i] = new_current_letter;
                    break;
                } else {
                }
            }
            // truncate the tone number
            chars.truncate(chars.len() - 1);
            Cow::Owned(chars.into_iter().collect())
        }
    });

    // Would have liked to use `intersperse` here to avoid the Vec but it isn't stable yet.
    let pretty_pinyin: String = syllables.collect::<Vec<_>>().join(" ");

    pretty_pinyin
}
