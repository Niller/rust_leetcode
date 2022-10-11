pub fn longest_common_prefix(strs: Vec<String>) -> String {
    
    let mut prefix = String::new();
        let mut i = 0;

        loop {  
            if strs[0].len() <= i {
                return prefix;
            }
            
            let char = strs[0].chars().nth(i).unwrap();
            for str in &strs  {
                if str.len() <= i {
                    return prefix;
                }

                if char != str.chars().nth(i).unwrap() {
                    return prefix;
                }
            }
            prefix.push(char);
            i += 1;
        }
}