// @author      ::  Preston Wang-Stosur-Bassett <http://stosur.info>
// @date        ::  December 8, 2017
// @description ::  This file takes pinyin with tone numbers and returns pinyin with tone marks

//! ### About
//! Turn pinyin written with tone numbers and turn it into pinyin with node marks. prettify_pinyin accepts input in the [CC-CEDICT](https://cc-cedict.org/wiki/format:syntax) pinyin format (space separated syllables with tone numbers at the end of each syllable), for example: "ni3 hao3" will get turned into "nǐ hǎo".
//!
//! ### Usage
//! ```rust
//! extern crate prettify_pinyin;
//!
//! use prettify_pinyin::prettify;
//!
//! let test = String::from("ma1 ma2 ma3 ma4 ma");
//! let formatted: String = prettify(test);
//!
//! println!("{}", formatted); // --> mā má mǎ mà ma
//! ```

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn prettify() {
        use prettify;

        let hello = String::from("nǐ hǎo");
        let china = String::from("zhōng guó");
        let all_tones = String::from("mā má mǎ mà");
        let no_tones = String::from("ma");
        let capital_letter = String::from("Ān huī");

        assert_eq!(hello, prettify(String::from("ni3 hao3")));
        assert_eq!(china, prettify(String::from("zhong1 guo2")));
        assert_eq!(all_tones, prettify(String::from("ma1 ma2 ma3 ma4")));
        assert_eq!(no_tones, prettify(String::from("ma")));
        assert_eq!(capital_letter, prettify(String::from("An1 hui1")));
    }
}

/// # prettify
/// ```
/// extern crate prettify_pinyin;
/// use prettify_pinyin::prettify;
/// prettify(String::from("ma1 ma2 ma3 ma4 ma")); // --> mā má mǎ mà ma
/// ```
pub fn prettify(raw: String) -> String {
    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
    replacements.insert(String::from("a"), vec![String::from("ā"), String::from("á"), String::from("ǎ"), String::from("à")]);
    replacements.insert(String::from("e"), vec![String::from("ē"), String::from("é"), String::from("ě"), String::from("è")]);
    replacements.insert(String::from("u"), vec![String::from("ū"), String::from("ú"), String::from("ǔ"), String::from("ù")]);
    replacements.insert(String::from("i"), vec![String::from("ī"), String::from("í"), String::from("ǐ"), String::from("ì")]);
    replacements.insert(String::from("o"), vec![String::from("ō"), String::from("ó"), String::from("ǒ"), String::from("ò")]);
    replacements.insert(String::from("ü"), vec![String::from("ǖ"), String::from("ǘ"), String::from("ǚ"), String::from("ǜ")]);
    replacements.insert(String::from("A"), vec![String::from("Ā"), String::from("Á"), String::from("Ă"), String::from("À")]);
    replacements.insert(String::from("E"), vec![String::from("Ē"), String::from("É"), String::from("Ĕ"), String::from("È")]);
    replacements.insert(String::from("U"), vec![String::from("Ū"), String::from("Ú"), String::from("Ŭ"), String::from("Ù")]);
    replacements.insert(String::from("I"), vec![String::from("Ī"), String::from("Í"), String::from("Ĭ"), String::from("Ì")]);
    replacements.insert(String::from("O"), vec![String::from("Ō"), String::from("Ó"), String::from("Ŏ"), String::from("Ò")]);
    replacements.insert(String::from("Ü"), vec![String::from("Ǖ"), String::from("Ǘ"), String::from("Ǚ"), String::from("Ǜ")]);

    let mut medials: HashMap<String, u8> = HashMap::new();
    medials.insert(String::from("i"), 0 as u8);
    medials.insert(String::from("u"), 0 as u8);
    medials.insert(String::from("ü"), 0 as u8);

    let mut syl_vec: Vec<String> = Vec::new();
    let text = raw.replace("v", "ü");
    let syllables: Vec<_> = text.split(" ").collect();

    for syllable in syllables {
        let tone: u8 = match syllable.chars().skip(syllable.chars().count() - 1).take(1).collect::<String>().parse::<u8>() {
            Ok(tone) => tone,
            Err(_e) => {
                syl_vec.push(syllable.to_string());
                continue;
            }
        };

        if tone <= 0 as u8 || tone > 5 as u8 {
            // This is not a valid number
            println!("Invalid tone number: {:?} in: {:?}", tone, syllable);
        } else if tone == 5 as u8 {
            let pretty_syl: String = syllable.chars().take(syllable.chars().count() - 1).collect();
            syl_vec.push(pretty_syl);
        } else {
            let mut j = 0 as u8;
            let mut done = false;

            while !done {
                let current_letter: String = syllable.chars().skip(j as usize).take(1).collect();
                let next_letter: String = syllable.chars().skip((j + (1 as u8)) as usize).take(1).collect();

                if replacements.contains_key(&current_letter) {
                    let to_replace;

                    if replacements.contains_key(&next_letter) && medials.contains_key(&current_letter) {
                        to_replace = next_letter;
                    } else {
                        to_replace = current_letter;
                    }

                    let replaced: String = syllable.replace(&to_replace, replacements.get(&to_replace).unwrap().get((tone - (1 as u8)) as usize).unwrap());
                    let pretty_syl: String = replaced.chars().take(replaced.chars().count() - 1).collect();
                    syl_vec.push(pretty_syl);
                    break;
                }

                j = j + (1 as u8);
                if j as usize == syllable.chars().count() {
                    done = true;
                }
            }
        }
    }

    let pretty_pinyin: String = syl_vec.join(" ");
    return pretty_pinyin;
}
