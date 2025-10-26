use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Normalizes text by expanding abbreviations and applying other text normalization rules
/// to improve TTS quality.
pub struct TextNormalizer {
    abbreviation_map: HashMap<String, String>,
}

impl TextNormalizer {
    /// Creates a new `TextNormalizer` instance with common English abbreviations.
    pub fn new() -> Self {
        Self {
            abbreviation_map: Self::initialize_abbreviation_map(),
        }
    }

    /// Normalizes the input text by expanding abbreviations and applying other rules.
    pub fn normalize(&self, text: &str) -> String {
        // Replace ’ with '
        let text = text.replace("’", "'");

        // Convert input text to lowercase
        let text = self.expand_abbreviations(&text);

        // Apply additional normalization rules
        let text = self.expand_contractions(&text);
        let text = self.normalize_numbers(&text);
        let text = self.normalize_symbols(&text);

        text
    }

    /// Expands abbreviations in the input text.
    fn expand_abbreviations(&self, text: &str) -> String {
        lazy_static! {
            // Match word boundaries to prevent partial word matches
            static ref WORD_BOUNDARY: Regex = Regex::new(r"\b([a-zA-Z.']+)\b").unwrap();
        }

        // Use regex to match words and check if they're in the abbreviation map
        WORD_BOUNDARY
            .replace_all(text, |caps: &regex::Captures| {
                let word = &caps[1];
                // Convert the word to lowercase for case-insensitive comparison
                if let Some(expansion) = self.abbreviation_map.get(&word.to_lowercase()) {
                    expansion.to_string()
                } else {
                    word.to_string()
                }
            })
            .to_string()
    }

    /// Expands common English contractions (don't -> do not, I've -> I have, etc.)
    fn expand_contractions(&self, text: &str) -> String {
        lazy_static! {
            static ref CONTRACTIONS: HashMap<&'static str, &'static str> = {
                let mut map = HashMap::new();

                // Negations
                map.insert("don't", "do not");
                map.insert("doesn't", "does not");
                map.insert("didn't", "did not");
                map.insert("won't", "will not");
                map.insert("wouldn't", "would not");
                map.insert("can't", "cannot");
                map.insert("cannot", "can not");
                map.insert("couldn't", "could not");
                map.insert("shouldn't", "should not");
                map.insert("shan't", "shall not");
                map.insert("mustn't", "must not");
                map.insert("mightn't", "might not");
                map.insert("mayn't", "may not");
                map.insert("needn't", "need not");
                map.insert("oughtn't", "ought not");
                map.insert("ain't", "am not");
                map.insert("isn't", "is not");
                map.insert("aren't", "are not");
                map.insert("wasn't", "was not");
                map.insert("weren't", "were not");
                map.insert("haven't", "have not");
                map.insert("hasn't", "has not");
                map.insert("hadn't", "had not");

                // Have contractions
                map.insert("I've", "I have");
                map.insert("you've", "you have");
                map.insert("we've", "we have");
                map.insert("they've", "they have");
                map.insert("could've", "could have");
                map.insert("should've", "should have");
                map.insert("would've", "would have");
                map.insert("must've", "must have");
                map.insert("might've", "might have");
                map.insert("may've", "may have");

                // Are/is/am contractions
                map.insert("I'm", "I am");
                map.insert("you're", "you are");
                map.insert("he's", "he is");
                map.insert("she's", "she is");
                map.insert("it's", "it is");
                map.insert("we're", "we are");
                map.insert("they're", "they are");
                map.insert("that's", "that is");
                map.insert("who's", "who is");
                map.insert("what's", "what is");
                map.insert("where's", "where is");
                map.insert("when's", "when is");
                map.insert("why's", "why is");
                map.insert("how's", "how is");
                map.insert("there's", "there is");
                map.insert("here's", "here is");

                // Will contractions
                map.insert("I'll", "I will");
                map.insert("you'll", "you will");
                map.insert("he'll", "he will");
                map.insert("she'll", "she will");
                map.insert("it'll", "it will");
                map.insert("we'll", "we will");
                map.insert("they'll", "they will");
                map.insert("that'll", "that will");

                // Would contractions
                map.insert("I'd", "I would");
                map.insert("you'd", "you would");
                map.insert("he'd", "he would");
                map.insert("she'd", "she would");
                map.insert("it'd", "it would");
                map.insert("we'd", "we would");
                map.insert("they'd", "they would");
                map.insert("that'd", "that would");

                // Had contractions (note: some of these overlap with would)
                // For simplicity, we're defaulting to the "would" interpretation
                // For more accuracy, context-sensitive parsing would be needed

                // Let us contractions
                map.insert("let's", "let us");

                map
            };
        }

        // Capitalize first letter of expanded contractions if the original was capitalized
        let mut result = text.to_string();
        for (contraction, expansion) in CONTRACTIONS.iter() {
            // Case-insensitive match for the contraction
            let pattern = format!(r"\b{}\b", regex::escape(contraction));
            let case_regex = Regex::new(&pattern).unwrap();

            result = case_regex
                .replace_all(&result, |caps: &regex::Captures| {
                    let matched = &caps[0];
                    // Check if the first letter is uppercase
                    if !matched.is_empty() && matched.chars().next().unwrap().is_uppercase() {
                        // Capitalize the first letter of the expansion
                        let mut chars = expansion.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first_char) => {
                                first_char.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        }
                    } else {
                        expansion.to_string()
                    }
                })
                .to_string();
        }

        result
    }

    /// Normalizes numbers in the input text (e.g., 123 -> one hundred twenty-three).
    fn normalize_numbers(&self, text: &str) -> String {
        lazy_static! {
            // Pattern for matching numbers
            static ref NUMBER_PATTERN: Regex = Regex::new(r"\b(\d+)\b").unwrap();
        }

        NUMBER_PATTERN
            .replace_all(text, |caps: &regex::Captures| {
                let number_str = &caps[1];
                match number_str.parse::<u64>() {
                    Ok(number) => {
                        // For simplicity, only handle numbers below a certain threshold
                        if number <= 999 {
                            self.number_to_words(number)
                        } else {
                            // For larger numbers, keep as is for now
                            number_str.to_string()
                        }
                    }
                    Err(_) => number_str.to_string(),
                }
            })
            .to_string()
    }

    /// Converts a number to its word representation.
    fn number_to_words(&self, number: u64) -> String {
        if number == 0 {
            return "zero".to_string();
        }

        let under_twenty = [
            "",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
        ];

        let tens = [
            "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
        ];

        if number < 20 {
            return under_twenty[number as usize].to_string();
        } else if number < 100 {
            let ten = tens[(number / 10) as usize];
            let unit = under_twenty[(number % 10) as usize];

            if unit.is_empty() {
                return ten.to_string();
            } else {
                return format!("{}-{}", ten, unit);
            }
        } else {
            let hundred = under_twenty[(number / 100) as usize];
            let remainder = number % 100;

            if remainder == 0 {
                return format!("{} hundred", hundred);
            } else {
                return format!("{} hundred {}", hundred, self.number_to_words(remainder));
            }
        }
    }

    /// Normalizes common symbols in the input text.
    fn normalize_symbols(&self, text: &str) -> String {
        let text = text.replace("&", " and ");
        let text = text.replace("%", " percent ");
        let text = text.replace("$", " dollars ");
        let text = text.replace("@", " at ");
        let text = text.replace("#", " number ");

        text
    }

    /// Initializes a map of common English abbreviations and their expansions.
    fn initialize_abbreviation_map() -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Common title abbreviations
        map.insert("Mr.".to_string(), "Mister".to_string());
        map.insert("Mrs.".to_string(), "Misses".to_string());
        map.insert("Ms.".to_string(), "Miss".to_string());
        map.insert("Dr.".to_string(), "Doctor".to_string());
        map.insert("Prof.".to_string(), "Professor".to_string());
        map.insert("Rev.".to_string(), "Reverend".to_string());

        // Time abbreviations
        map.insert("sec.".to_string(), "second".to_string());
        map.insert("min.".to_string(), "minute".to_string());
        map.insert("hr.".to_string(), "hour".to_string());
        map.insert("hrs.".to_string(), "hours".to_string());

        // Days of the week
        map.insert("Mon.".to_string(), "Monday".to_string());
        map.insert("Tue.".to_string(), "Tuesday".to_string());
        map.insert("Wed.".to_string(), "Wednesday".to_string());
        map.insert("Thu.".to_string(), "Thursday".to_string());
        map.insert("Fri.".to_string(), "Friday".to_string());
        map.insert("Sat.".to_string(), "Saturday".to_string());
        map.insert("Sun.".to_string(), "Sunday".to_string());

        // Months
        map.insert("Jan.".to_string(), "January".to_string());
        map.insert("Feb.".to_string(), "February".to_string());
        map.insert("Mar.".to_string(), "March".to_string());
        map.insert("Apr.".to_string(), "April".to_string());
        map.insert("Jun.".to_string(), "June".to_string());
        map.insert("Jul.".to_string(), "July".to_string());
        map.insert("Aug.".to_string(), "August".to_string());
        map.insert("Sep.".to_string(), "September".to_string());
        map.insert("Sept.".to_string(), "September".to_string());
        map.insert("Oct.".to_string(), "October".to_string());
        map.insert("Nov.".to_string(), "November".to_string());
        map.insert("Dec.".to_string(), "December".to_string());

        // Common abbreviations
        map.insert("e.g.".to_string(), "for example".to_string());
        map.insert("i.e.".to_string(), "that is".to_string());
        map.insert("etc.".to_string(), "etcetera".to_string());
        map.insert("vs.".to_string(), "versus".to_string());
        map.insert("St.".to_string(), "Street".to_string());
        map.insert("Ave.".to_string(), "Avenue".to_string());
        map.insert("Blvd.".to_string(), "Boulevard".to_string());
        map.insert("Rd.".to_string(), "Road".to_string());
        map.insert("approx.".to_string(), "approximately".to_string());
        map.insert("appt.".to_string(), "appointment".to_string());
        map.insert("apt.".to_string(), "apartment".to_string());
        map.insert("asap".to_string(), "as soon as possible".to_string());
        map.insert("ASAP".to_string(), "as soon as possible".to_string());

        // Common text messaging and internet abbreviations
        map.insert("btw".to_string(), "by the way".to_string());
        map.insert("BTW".to_string(), "by the way".to_string());
        map.insert("lol".to_string(), "laughing out loud".to_string());
        map.insert("LOL".to_string(), "laughing out loud".to_string());
        map.insert("afaik".to_string(), "as far as I know".to_string());
        map.insert("AFAIK".to_string(), "as far as I know".to_string());
        map.insert("idk".to_string(), "I don't know".to_string());
        map.insert("IDK".to_string(), "I don't know".to_string());
        map.insert("fyi".to_string(), "for your information".to_string());
        map.insert("FYI".to_string(), "for your information".to_string());

        // Academic and publication abbreviations
        map.insert("fig.".to_string(), "figure".to_string());
        map.insert("Ch.".to_string(), "Chapter".to_string());
        map.insert("pg.".to_string(), "page".to_string());
        map.insert("pp.".to_string(), "pages".to_string());
        map.insert("vol.".to_string(), "volume".to_string());
        map.insert("ed.".to_string(), "edition".to_string());

        // Measurements
        map.insert("km".to_string(), "kilometers".to_string());
        map.insert("cm".to_string(), "centimeters".to_string());
        map.insert("mm".to_string(), "millimeters".to_string());
        map.insert("mg".to_string(), "milligrams".to_string());
        map.insert("kg".to_string(), "kilograms".to_string());
        map.insert("ft".to_string(), "feet".to_string());
        map.insert("in".to_string(), "inches".to_string());
        map.insert("lbs".to_string(), "pounds".to_string());

        map
    }

    /// Add a custom abbreviation to the normalizer.
    pub fn add_abbreviation(&mut self, abbreviation: &str, expansion: &str) {
        self.abbreviation_map
            .insert(abbreviation.to_string(), expansion.to_string());
    }
}
