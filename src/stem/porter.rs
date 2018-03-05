use regex::Regex;

pub struct Stemmer {
    j: usize,
    k: usize,
    buffer: Vec<u8>,
}

impl Stemmer {

    pub fn new(s: &str) -> Self {
        Stemmer {
            buffer: String::from(s).into_bytes(),
            j: 0,
            k: s.len(),
        }
    }

    fn cons(&self, i: usize) -> bool {
        match self.buffer[i] {
            b'a' | b'e' | b'i' | b'o' | b'u' => false,
            b'y' => if i == 0 { true } else { !self.cons(i-1) },
            _   => true,
        }
    }

    fn measure(&self) -> usize {
        let mut n = 0;
        let mut i = 0;
        let mut next = (i..self.j).find(|&i| !self.cons(i));
        
        while i < self.j {
            if next.is_none() { break } else { i = next.unwrap(); }
            next = (i..self.j).find(|&i| self.cons(i));
            if next.is_none() { break } else { n += 1; i = next.unwrap(); }
            next = (i..self.j).find(|&i| !self.cons(i));
        }

        return n;
    }

    fn contains_vowel(&self) -> bool {
        (0..self.j).any(|i| !self.cons(i))
    }

    fn double_cons(&self, i: usize) -> bool {
        if i < 1 || (self.buffer[i] != self.buffer[i - 1]) {
            false 
        } else {
            self.cons(i)
        }
    }

    fn cons_v_cons(&self, i: usize) -> bool {
        if i < 2 || !self.cons(i) || self.cons(i - 1) || self.cons(i - 2) {
            return false
        }
        match self.buffer[i] {
            b'w' | b'x' | b'y' => false,
            _                  => true,
        }
    }

    fn ends(&mut self, s: &[u8]) -> bool {
        let l = s.len(); 
        if s[l] != self.buffer[self.k] || l > self.k + 1 || !self.buffer.ends_with(s) {
            false 
        } else {
            self.j = self.k - l; 
            true
        }
    }

    fn set(&mut self, s: &[u8]) {
        let l = s.len();
        let r = (self.j + 1)..(self.buffer.len());
        self.buffer.splice(r, s.iter().cloned());
        self.k = self.j + l;
    }

    fn replace(&mut self, s: &[u8]) {
        if self.measure() > 0 {
            self.set(s);
        }
    }

    pub fn stem(self) -> String {
        return String::from(""); 

    }
}
