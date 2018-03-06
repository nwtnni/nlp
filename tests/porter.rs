extern crate nlp;

use nlp::stem::porter::Porter;

const DICTIONARY: &'static str = include_str!("dict.txt");
const REFERENCE: &'static str = include_str!("stemmed.txt");

#[test]
fn test_stemmer() {
    
    let input = DICTIONARY.split("\n");
    let output = REFERENCE.split("\n");
     
    for (i, o) in input.zip(output) {
        let i = i.trim_right(); 
        let o = o.trim_right();
    
        let s = Porter::stem(i);
        assert!(s.is_ok());
        assert_eq!(s.unwrap(), o);
    }
}

