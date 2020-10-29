extern crate rand;

// use rand::thread_rng;
// use rand::Rng;
use std::fs::File;
use std::io::Read;
// use std::error::Error;

struct WordList {
    words: Vec<String>,
    wordCt: u32
}

impl WordList {
    // returns a random string from self.words
    fn randWord(&self) -> String {
        let ndx: i32 = rand::random() % self.wordCt();
        return words[ndx];
    }

    // generates a passphrase (3 words first letters capitalized)
    fn genPassphrase(&self){
        let mut rv = String::new();
    }

    // prints the word list to the screen for testing purposes
    fn dump(&self) {
        for word in self.words.iter(){
            println!("{}", word);
        }
    }

    fn new(listFile: String) -> WordList {
        let mut file = File::open(listFile).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        let mut vec: Vec<String> = Vec::new();
        let mut ct: u32 = 0;
        for word in contents.lines(){
            vec.push(word.to_string());
            ct += 1;
        }

        let rv = WordList {
            words: vec,
            wordCt: ct
        };

        return rv;
    }
}


fn main(){
    let path = "test1.txt";

    let testList = WordList::new(path);
    testList.dump();
}
