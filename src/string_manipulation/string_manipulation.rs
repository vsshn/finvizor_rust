pub fn find_between<'a>(text: &'a str, start_word: &str, end_word: &str) -> Option<&'a str> {
    // Find the position of the start word
    let start_pos = text.find(start_word)? + start_word.len();
    // Find the position of the end word after the start word
    let end_pos = text[start_pos..].find(end_word)? + start_pos;
    // Return the substring between them
    Some(&text[start_pos..end_pos])
}
