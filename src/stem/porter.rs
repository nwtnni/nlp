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

    fn double_cons(&self) -> bool {
        if self.k < 1 || (self.buffer[self.k] != self.buffer[self.k - 1]) {
            false 
        } else {
            self.cons(self.k)
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

    fn ends(&mut self, s: &str) -> bool {
        let l = s.len(); 
        if l > self.k + 1 || !self.buffer.ends_with(s.as_bytes()) {
            false 
        } else {
            self.j = self.k - l; 
            true
        }
    }

    fn set(&mut self, s: &str) {
        let l = s.len();
        let r = (self.j + 1)..(self.buffer.len());
        self.buffer.splice(r, s.bytes());
        self.k = self.j + l;
    }

    fn replace(&mut self, s: &str) {
        if self.measure() > 0 {
            self.set(s);
        }
    }

    fn step1ab(&mut self) {

        if self.buffer[self.k] == b's' {
            if self.ends("sses") { self.k -= 2; }
            else if self.ends("ies") { self.set("i"); }
            else if self.buffer[self.k - 1] != b's' { self.k -= 1; }
        }

        if self.ends("eed") { if self.measure() > 0 { self.k -= 1; } }
        else if (self.ends("ed") || self.ends("ing")) && self.contains_vowel() {
            self.k = self.j;

            if self.ends("at") { self.set("ate"); }
            else if self.ends("bl") { self.set("ble"); }
            else if self.ends("iz") { self.set("ize"); }
            else if self.double_cons() {
                self.k -= 1; 
                match self.buffer[self.k] {
                    b'l' | b's' | b'z' => self.k += 1,
                    _                  => (),
                }
            }
            else if self.measure() == 1 && self.cons_v_cons(self.k) {
                self.set("e");
            }
        }
    }

    pub fn stem(self) -> String {
        return String::from(""); 
    }
}
