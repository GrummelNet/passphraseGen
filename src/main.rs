#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]

extern crate rand;

struct Password {
    text: String,
    wordCt: usize
}

impl Password {
    fn len(&self) -> usize {
        return self.text.len()
    }

    // currently returns the entropy given lowercase english letters
    fn stats(&self, wl: &WordList) -> (f64, f64, f64) {
        // let alph: f64 = 26.0;
        let lower = (self.text.len() as f64) * (26.0 as f64).log2();
        let allEng = (self.text.len() as f64) * (52.0 as f64).log2();
        let dict = (self.wordCt as f64) * (wl.wordCt as f64).log2();

		return (lower, allEng, dict);
    }

    fn new(str: String, n: usize) -> Password {
        let rv = Password {
            text: str,
            wordCt: n
        };

        return rv;
    }
}

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
    fn genPassphrase(&self) -> Password{
        let mut phrase = String::new();
        for _i in 0..3{
            let word = self.randWord();
            phrase += &word
        }

        let mut wordCt = 3;
        // ensuring a suitable length passphrase
        while phrase.len() < 15 {
            let word = self.randWord();
            phrase += &word;
            wordCt = wordCt + 1;
        }

        let rv = Password::new(phrase, wordCt);
        return rv;
    }

    // prints the word list to the screen for testing purposes
    fn dump(&self) {
        for word in self.words.iter(){
            println!("{}", word);
        }
    }

    fn dumpStats(&self) {
        println!("Number of words: \t{}", self.wordCt);
    }

    fn new() -> WordList {
        // let mut contents = include_str!("test1.txt");
        let contents = include_str!("twl06.txt");
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

// produces a list of 10 passphrases
fn listOfTen(corpus: WordList) {
    //making a list of passphrases
    let mut passes: Vec<Password> = Vec::new();
    let mut max: usize = 0;
    for i in 0..10 {
        passes.push(corpus.genPassphrase());
        if passes[i].len() > max {
            max = passes[i].len();
        }
    }

    // funky formatting for a table output
    println!("{ul}Num | {:41}| {:18}{res}", "Passphrase" , "Entropy " , ul="\x1B[4m", res="\x1B[24m");
    for i in 0..10 {
		let stats = passes[i].stats(&corpus);
        println!("[{}] | {:41}| {}", i, passes[i].text, stats.0); 
    }
}

fn main(){
    // let testList = WordList::new(testPath);
    let corpus = WordList::new();

    listOfTen(corpus);
    // corpus.dumpStats();
}
