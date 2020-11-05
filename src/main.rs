extern crate rand;

// use rand::thread_rng;
// use rand::Rng;
use std::fs::File;
use std::io::Read;
// use std::error::Error;

struct WordList {
    words: Vec<String>,
    wordCt: usize
}

impl WordList {
    // returns a random string from self.words
    fn randWord(&self) -> String {
        let ndx: usize = rand::random::<usize>() % self.wordCt;
        // capitalizing first letter
        let w = self.words[ndx].clone();
        let rv = format!("{}{}", w[..1].to_string().to_uppercase(), w[1..].to_string()).to_string();
        return rv;
    }

    // generates a passphrase (3 words first letters capitalized)
    fn genPassphrase(&self) -> String{
        let mut rv = String::new();
        for i in 0..3{
            let mut word = self.randWord();
            rv += &word;
        }
        return rv;
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
        let mut ct: usize = 0;
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
    let testPath = "test1.txt".to_string();
    let path = "twl06.txt".to_string();

    let testList = WordList::new(testPath);
    let corpus = WordList::new(path);

    for i in 0..10{
        println!("{}", corpus.genPassphrase());
    }
}
