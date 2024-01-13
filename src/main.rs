// Define the WordCounter struct to hold the text to be analyzed
struct WordCounter {
   text: String,
}

// Implement methods for the WordCounter struct
impl WordCounter {
   // Create a new WordCounter instance from a given text
   fn new(text: &str) -> WordCounter {
       WordCounter {
           text: text.to_string(), // Store a full String copy for ownership
       }
   }

   // Count the number of words in the text
   fn count_words(&self) -> usize {
       self.text.split_whitespace().count() // Split text by whitespace and count words
   }
}

fn main() {
   // Prompt the user to enter text
   println!("Enter a text:");

   // Read the user's input
   let mut input = String::new();
   std::io::stdin().read_line(&mut input).expect("Failed to read input"); // Handle potential errors

   // Create a WordCounter instance with the user's text
   let word_counter = WordCounter::new(&input);

   // Count the words in the text
   let word_count = word_counter.count_words();

   // Print the word count to the console
   println!("The text has {} words.", word_count);
}

