use proconio::input;

fn check(s: &str) -> bool {
    let mut ret: bool = true;
    let mut status: isize = 0;
    for c in s.chars() {
        if c == '(' {
            status += 1;
        } else if c == ')' {
            status -= 1;
        }
        if status < 0 {
            ret = false;
            break;
        }
    }
    if status != 0 {
        ret = false;
    }
    return ret;
}

fn main() {
    input! {
        n: usize,
    }

    let pow_n: usize = 1 << n;
    for i in (0..pow_n).rev() {
        // println!("{}", i);
        let mut s: String = String::from("");
        for j in (0..n).rev() {
            if (i & (1 << j)) == 0 {
                s.push(')');
            } else {
                s.push('(');
            }
        }
        if check(&s.to_string()) == true {
            println!("{}", s);
        }
    }
}
