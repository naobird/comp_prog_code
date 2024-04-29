use proconio::input;

fn check_score(v: &Vec<isize>, peaces: isize, score: isize) -> bool {
    let mut count = 0;
    let mut temp_sum = 0;
    for i in 0..v.len() {
        temp_sum += v[i];
        if temp_sum >= score {
            count += 1;
            temp_sum = 0;
        }
    }
    return count >= peaces;
}

fn binary_search(v: &Vec<isize>, peaces: isize, max_value: isize) -> isize {
    let mut ok: isize = -1;
    let mut ng: isize = max_value + 1;
    loop {
        if (ok - ng).abs() <= 1 {
            break;
        }
        let middle = (ok + ng) / 2;
        if check_score(&v, peaces, middle) {
            ok = middle;
        } else {
            ng = middle;
        }
    }
    return ok;
}

fn main() {
    input! {
        n: usize,
        l: isize,
        k: isize,
        a: [isize;n],
    }
    let mut len: Vec<isize> = Vec::with_capacity(n);
    len.push(a[0]);
    for i in 1..n {
        len.push(a[i] - a[i - 1]);
    }
    len.push(l - a[n - 1]);
    println!("{}", binary_search(&len, k + 1, l));
}
