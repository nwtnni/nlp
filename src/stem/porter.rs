pub struct Stemmer {
    buffer  : String,
    offset  : usize,
    current : usize,
}

impl Stemmer {

    pub fn new(s: &str) -> Self {
        Stemmer {
            buffer: String::from(s),
            offset: s.len(),
            current: 0,
        }
    }

    fn is_cons(&self, index: usize) -> bool {
        unsafe {
            match self.buffer.get_unchecked(index..index+1) {
                "a" | "e" | "i" | "o" | "u" => false,
                "y" => if index == 0 { true } else { !self.is_cons(index - 1) },
                _   => true,
            }
        }
    }

    fn measure(&self) -> usize {
        let mut n = 0; 
        let mut i = 0;
        
        
        return 0;
    }


    pub fn stem(self) -> String {
        return String::from(""); 

    }
}
