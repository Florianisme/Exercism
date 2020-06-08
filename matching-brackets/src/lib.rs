pub fn brackets_are_balanced(string: &str) -> bool {
    let mut opened_brackets = vec![];
    for char in string.chars() {
        if char_matches_opening_bracket(char) {
            opened_brackets.push(char);
        } else if char_matches_closing_bracket(char) {
            let last_opened_bracket = opened_brackets.last().copied();
            if last_opened_bracket.is_none() {
                return false;
            }

            if closing_bracket_matches_last_opened_bracket(char, last_opened_bracket.unwrap()) {
                opened_brackets.pop();
            } else {
                return false;
            }
        }
    }
    opened_brackets.is_empty()
}

fn closing_bracket_matches_last_opened_bracket(closing_char: char, last_opened_bracket: char) -> bool {

    match closing_char {
        '}' => last_opened_bracket.eq(&'{'),
        ')' => last_opened_bracket.eq(&'('),
        ']' => last_opened_bracket.eq(&'['),
         _  => false
    }
}

fn char_matches_opening_bracket(char: char) -> bool {
    char.eq(&'{') || char.eq(&'(') || char.eq(&'[')
}

fn char_matches_closing_bracket(char: char) -> bool {
    char.eq(&'}') || char.eq(&')') || char.eq(&']')
}
