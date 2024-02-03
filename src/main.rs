use std::io;

struct WordCounter {
    text: String,
}

impl WordCounter {
    //Constructor
    fn new() -> WordCounter {
        WordCounter {
            text: String::new(),
        }
    }

    //Reading texts
    fn read_text(&mut self) {
        println!("Enter some text:");
        io::stdin()
            .read_line(&mut self.text)
            .expect("Failed to read line");
    }

    //Counting words
    fn count_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}
