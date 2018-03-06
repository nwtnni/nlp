pub struct Porter {
    j: usize,
    k: usize,
    b: Vec<u8>,
}

impl Porter {
    pub fn stem(s: &str) -> Result<String, String> {
        Self::new(s).map(|stemmer| stemmer.execute())
    }

    fn new(s: &str) -> Result<Self, String> {
        if s.is_ascii() {
            Ok(Porter {
                b: s.to_lowercase().into_bytes(),
                j: 0,
                k: s.len() - 1,
            })
        } else {
            Err(String::from("Porter stemmer only supports ASCII strings"))
        }
    }

    fn cons(&self, i: usize) -> bool {
        match self.b[i] {
            b'a' | b'e' | b'i' | b'o' | b'u' => false,
            b'y' => if i == 0 { true } else { !self.cons(i - 1) },
            _   => true,
        }
    }

    fn measure(&self) -> usize {
        let mut n = 0;
        let mut i = 0;
        let j = self.j + 1;
        let mut next = (i..j).find(|&i| !self.cons(i));
        
        while i < j {
            if next.is_none() { break } else { i = next.unwrap(); }
            next = (i..j).find(|&i| self.cons(i));
            if next.is_none() { break } else { n += 1; i = next.unwrap(); }
            next = (i..j).find(|&i| !self.cons(i));
        }

        n
    }

    fn contains_vowel(&self) -> bool {
        (0..self.j + 1).any(|i| !self.cons(i))
    }

    fn double_cons(&self) -> bool {
        if self.k < 1 || (self.b[self.k] != self.b[self.k - 1]) {
            false 
        } else {
            self.cons(self.k)
        }
    }

    fn cons_vowel_cons(&self, i: usize) -> bool {
        if i < 2 || !self.cons(i) || self.cons(i - 1) || !self.cons(i - 2) {
            return false
        }
        match self.b[i] {
            b'w' | b'x' | b'y' => false,
            _                  => true,
        }
    }

    fn ends(&mut self, s: &str) -> bool {
        let l = s.len(); 
        let b = s.as_bytes();
        if l > self.k || &self.b[self.k + 1 - l..self.k + 1] != b {
            false 
        } else {
            self.j = self.k - l;
            true
        }
    }

    fn set(&mut self, s: &str) {
        let l = s.len();
        let r = (self.j + 1)..(self.k + 1);
        self.b.splice(r, s.bytes());
        self.k = self.j + l;
    }

    fn replace(&mut self, s: &str) {
        if self.measure() > 0 { self.set(s) }
    }

    fn step_1ab(&mut self) {

        if self.b[self.k] == b's' {
            if      self.ends("sses")          { self.k -= 2   }
            else if self.ends("ies")           { self.set("i") }
            else if self.b[self.k - 1] != b's' { self.k -= 1   }
        }

        if self.ends("eed") {
            if self.measure() > 0 {
                self.k -= 1
            }
        } else if (self.ends("ed") || self.ends("ing")) && self.contains_vowel() {
            self.k = self.j;

            if      self.ends("at") { self.set("ate") }
            else if self.ends("bl") { self.set("ble") }
            else if self.ends("iz") { self.set("ize") }
            else if self.double_cons() {
                self.k -= 1; 
                match self.b[self.k] {
                    b'l' | b's' | b'z' => self.k += 1,
                    _                  => (),
                }
            }
            else if self.measure() == 1 && self.cons_vowel_cons(self.k) {
                self.set("e")
            }
        }
    }

    fn step_1c(&mut self) {
        if self.ends("y") && self.contains_vowel() {
            self.b[self.k] = b'i';
        }
    }

    fn step_2(&mut self) {
        match self.b[self.k - 1] {
            b'a' => if      self.ends("ational") { self.replace("ate")  }
                    else if self.ends("tional")  { self.replace("tion") },
            b'c' => if      self.ends("enci")    { self.replace("ence") }
                    else if self.ends("anci")    { self.replace("ance") },
            b'e' => if      self.ends("izer")    { self.replace("ize")  },
            b'l' => if      self.ends("bli")     { self.replace("ble")  }
                    else if self.ends("alli")    { self.replace("al")   }
                    else if self.ends("entli")   { self.replace("ent")  }
                    else if self.ends("eli")     { self.replace("e")    }
                    else if self.ends("ousli")   { self.replace("ous")  },
            b'o' => if      self.ends("ization") { self.replace("ize")  }
                    else if self.ends("ation")   { self.replace("ate")  }
                    else if self.ends("ator")    { self.replace("ate")  },
            b's' => if      self.ends("alism")   { self.replace("al")   }
                    else if self.ends("iveness") { self.replace("ive")  }
                    else if self.ends("fulness") { self.replace("ful")  }
                    else if self.ends("ousness") { self.replace("ous")  },
            b't' => if      self.ends("aliti")   { self.replace("al")   }
                    else if self.ends("iviti")   { self.replace("ive")  }
                    else if self.ends("biliti")  { self.replace("ble")  },
            b'g' => if      self.ends("logi")    { self.replace("log")  },
            _ => (),
        }
    }
    
    fn step_3(&mut self) {
        match self.b[self.k] {
            b'e' => if      self.ends("icate") { self.replace("ic") }
                    else if self.ends("ative") { self.replace("")   }
                    else if self.ends("alize") { self.replace("al") },
            b'i' => if      self.ends("iciti") { self.replace("ic") },
            b'l' => if      self.ends("ical")  { self.replace("ic") }
                    else if self.ends("ful")   { self.replace("")   },
            b's' => if      self.ends("ness")  { self.replace("")   },
            _    => (),
        }
    }

    fn step_4(&mut self) {
        match self.b[self.k - 1] {
            b'a' => if !(self.ends("al"))         { return },
            b'c' => if !(self.ends("ance")
                    ||   self.ends("ence"))       { return },
            b'e' => if !(self.ends("er"))         { return },
            b'i' => if !(self.ends("ic"))         { return },
            b'l' => if !(self.ends("able")
                    ||   self.ends("ible"))       { return },
            b'n' => if !(self.ends("ant")
                    ||   self.ends("ement") 
                    ||   self.ends("ment")
                    ||   self.ends("ent"))        { return },
            b'o' => if !((self.ends("ion")
                    &&  (self.b[self.j] == b's'
                    ||   self.b[self.j] == b't'))
                    ||   self.ends("ou"))         { return },
            b's' => if !(self.ends("ism"))        { return },
            b't' => if !(self.ends("ate")
                    ||   self.ends("iti"))        { return },
            b'u' => if !(self.ends("ous"))        { return },
            b'v' => if !(self.ends("ive"))        { return },
            b'z' => if !(self.ends("ize"))        { return },
            _    => return,
        }
        if self.measure() > 1 { self.k = self.j }
    }

    fn step_5(&mut self) {
        self.j = self.k; 
        
        if self.b[self.k] == b'e' {
            let m = self.measure(); 
            if m > 1 || m == 1 && !self.cons_vowel_cons(self.k - 1) { 
                self.k -= 1;
            }
        }

        if self.b[self.k] == b'l' && self.double_cons() && self.measure() > 1 {
            self.k -= 1; 
        }
    }

    fn execute(mut self) -> String {
        let b = if self.k <= 1 {
            self.b
        } else {
            self.step_1ab(); 
            if self.k > 0 {
                self.step_1c(); 
                self.step_2();
                self.step_3();
                self.step_4();
                self.step_5();
            }
            self.b[0..self.k+1].to_vec()
        };
        unsafe {
            String::from_utf8_unchecked(b)
        }
    }
}
