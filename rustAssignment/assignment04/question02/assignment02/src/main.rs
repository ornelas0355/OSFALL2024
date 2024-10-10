fn most_frequent_word(text: &str) -> (String, usize) {
    // Split the text into words
    let words: Vec<&str> = text.split_whitespace().collect();
    
    // We assume N is the number of unique words in the input.
    let mut unique_words: [&str; 10] = [""; 10]; // Fixed-size array, that is an array of string slices, adjust size accordingly
                                                // initializes the array with 10 empty string slices
    let mut counts: [usize; 10] = [0; 10];   //an array type with a fixed size of 10 elements, where each element is of type usize 
                                            //(an unsigned integer type, used for indexing and sizes
    
    let mut word_count = 0; // Number of unique words so far

    for word in words {
        let mut found = false;  // initializes a mutable boolean variable named found and sets its initial value to false
        
        // Search for the word in unique_words array
        for i in 0..word_count {
            if unique_words[i] == word {
                counts[i] += 1;
                found = true;       // Set found to true if the word is found
                break;
            }
        }

        // If the word is not found, add it to the unique_words array
        if !found && word_count < 10 {
            unique_words[word_count] = word;
            counts[word_count] = 1;
            word_count += 1;
        }
    }

    // Find the word with the maximum count
    let mut max_word = unique_words[0];
    let mut max_count = counts[0];

    for i in 1..word_count {
        if counts[i] > max_count {
            max_word = unique_words[i];
            max_count = counts[i];
        }
    }

    (max_word.to_string(), max_count) // Return the word with the highest frequency
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
