fn sum(ns: &[u32]) -> Option<u32> {
    let mut s: u32 = 0;
    let mut overflow = false;
    for n in ns {
        match s.checked_add(*n) {
            None => {
                overflow = true;
                break;
            }
            Some(v) => {
                s = v;
            }
        }
    }

    if overflow {
        return None;
    }
    Some(s)
}

fn main() {
    let ns1 = vec![1, 3, 5, 7, 9];
    let ns2 = vec![1, 2, 3, 4, u32::MAX];
    println!("{:?}", sum(&ns2));
    println!("{:?}", sum(&ns1));
}
