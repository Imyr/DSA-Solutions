pub fn is_match(s: String, p: String) -> bool {
    let pattern: Vec<char> = p.chars().collect();
    let p_len = pattern.len();
    let string: Vec<char> = s.chars().collect();
    let s_len = string.len();
    let mut string_index: usize = 0;
    match p_len {
        0 => return false,
        1 => {
            if s_len==1 && pattern[0]==string[0] {
                return true;
            } else {
                return false;
            }
        },
        _ => {
            for i in 0..p_len {
                println!("{}", string_index);
                print!("{} - ", pattern[i]);
                if string_index>=s_len {
                    return false;
                }
                match pattern[i] {
                    '.' => {
                        println!(".({})", string[string_index]);
                        if i+1<p_len {
                            match pattern[i+1] {
                                '*' => continue,
                                _ => string_index+=1,
                            }
                        } else {
                            string_index+=1;
                        }
                    },
                    '*' => {
                        println!("*({})", string[string_index]);
                        match pattern[i-1] {
                            '.' => {
                                if i+1>=p_len{
                                    return true;
                                } else {
                                    while string_index<s_len && i+1<p_len && string[string_index]!=pattern[i+1] {
                                        string_index+=1;
                                    }
                                }
                            },
                            _ => {
                                if string[string_index]==pattern[i-1]{
                                    string_index+=1;
                                } else {
                                    continue;
                                }
                                while string_index<s_len && string[string_index]==pattern[i-1] {
                                    string_index+=1;
                                    if i+1<p_len && string_index<s_len && pattern[i+1]!=string[string_index] {
                                        continue;
                                    } else {
                                        break;
                                    }
                                }
                            },
                        }

                    },
                    _ => {
                        println!("_({})", string[string_index]);
                        if i+1<p_len { 
                            match pattern[i+1] {
                                '*' => continue,
                                _ => {
                                    if string[string_index]==pattern[i] {
                                        string_index+=1;
                                    } else {
                                        return false;
                                    }
                                }, 
                            }
                        } else {
                            if string[string_index]==pattern[i] {
                                string_index+=1;
                            } else {
                                return false;
                            }
                        }
                    },
                };
            }
            println!("{}", string_index);
            if string_index<s_len {
                return false;
            } else {
                return true;
            }
        }
    }   
}