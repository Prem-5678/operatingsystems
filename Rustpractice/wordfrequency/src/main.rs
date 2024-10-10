fn most_frequent_word(text: &str) -> (String, usize) {
    // Split the input text into words
    let words: Vec<&str> = text.split_whitespace().collect();
    
    // Variables to keep track of the most frequent word and its count
    let mut max_word = String::new();
    let mut max_count = 0;

    // Iterate through the list of words
    for i in 0..words.len() {
        let mut count = 0;
        
        // Count occurrences of the current word in the array
        for j in 0..words.len() {
            if words[i] == words[j] {
                count += 1;
            }
        }

        // Update the most frequent word and count if a higher count is found
        if count > max_count {
            max_count = count;
            max_word = words[i].to_string();
        }
    }

    (max_word, max_count) // return the most frequent word and its count
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
