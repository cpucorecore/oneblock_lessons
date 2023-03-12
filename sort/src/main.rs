use std::cmp::Ordering;

fn sort<T:PartialOrd+Copy>(items: &mut [T]) {
    for i in 0..items.len()-1 {
        let end_index = items.len() - i - 1;
        for j in 0..end_index {
            if items[j].ge(&items[j+1]) {
                let tmp = items[j];
                items[j] = items[j+1];
                items[j+1] = tmp;
            }
        }
    }
}

#[test]
fn test_sort() {
    let mut i32s = vec![3,8,8,1,9];
    sort(&mut i32s);
    assert_eq!(vec![1,3,8,8,9], i32s);
}

#[derive(Copy, Clone, Debug)]
struct Person {
    age: i8,
}

impl PartialEq<Self> for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

impl PartialOrd  for Person{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.age.cmp(&other.age))
    }
}

#[test]
fn person_sort_test() {
    let mut persons = vec![
        Person{age: 38 },
        Person{age: 28 }
    ];
    sort(&mut persons);
    dbg!(persons);
}

fn main() {
    let mut i32s = vec![3,8,8,1,9];
    sort(&mut i32s);
    dbg!(i32s);

    let mut persons = vec![
        Person{age: 38 },
        Person{age: 18 },
        Person{age: 8 },
        Person{age: 28 }
    ];
    sort(&mut persons);
    dbg!(persons);
}
