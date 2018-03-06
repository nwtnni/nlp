extern crate nlp;

use nlp::stem::porter::Porter;

const DICTIONARY: &'static str = include_str!("../resources/dict.txt");
const REFERENCE: &'static str = include_str!("../resources/stemmed.txt");

#[test]
fn test_stemmer() {
    
    let input = DICTIONARY.split("\n");
    let expected = REFERENCE.split("\n");
     
    for (i, e) in input.zip(expected) {
        let i = i.trim_right(); 
        let e = e.trim_right();
    
        let s = Porter::stem(i);
        assert!(s.is_ok());
        assert_eq!(s.unwrap(), e);
    }
}
