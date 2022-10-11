pub fn break_palindrome(mut palindrome: String) -> String {
    let len = palindrome.len();

    if len == 1 {
        return String::new();
    }
    
    let chars = palindrome.as_bytes();

    let mut t = 10000;
    for i in 0..len {

        if len%2 == 1 && i == len/2 {
            continue;
        }

        let char = chars[i];
        if char == 97 {
            if i == len-1 {
                t = i;
            }
            continue;
        }

        t = i;
        break;
    }

    if t == 10000 {
        return String::new();
    }

    if t == len-1 {
        palindrome.replace_range(t..t+1, "b");
    } else {
        palindrome.replace_range(t..t+1, "a");
    }

    palindrome
}