fn is_match(s: String, p: String) -> bool {
    let mut count = 0; 

    for (i, ch) in p.chars().enumerate() {
        if i < s.len() {
            let cur = s.chars().nth(i).unwrap();

            if ch == cur || ch == '?' {
                count += 1;
            } else if ch == '*' {
                if i == p.len() - 1 {
                    count += s.len() - i;
                    break;
                }

                count += 1;
            }
        }
    }

    if count == s.len() {
        return true;
    }

    return false;
}

fn main() {
  let s = "adceb".to_string();
  let p = "*a*b".to_string();

  let res = is_match(s, p);

  println!("{}", res);
}