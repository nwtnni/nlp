    fn is_cons(s: &[u8], i: usize) -> bool {
        match s[i] {
            b'a' | b'e' | b'i' | b'o' | b'u' => false,
            b'y' => if i == 0 { true } else { !is_cons(s, i-1) },
            _   => true,
        }
    }

pub fn main() { 
    
    let test = "ccccccacacaca";
    let buffer = test.as_bytes();
    let j = test.len();
    let mut n = 0;
    let mut i = 0;
    let mut next = (i..j).find(|&i| !is_cons(&buffer, i));

    while i < j {
        if next.is_none() { break; } else { i = next.unwrap(); }
        next = (i..j).find(|&i| is_cons(&buffer, i));
        if next.is_none() { break; } else { n += 1; i = next.unwrap(); }
        next = (i..j).find(|&i| !is_cons(&buffer, i));
    }
    
    println!("{}", n);
}
