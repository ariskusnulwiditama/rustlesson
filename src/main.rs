fn main() {
    println!("Hello, world!");
    let mut arr = [5, 3, 2, 4, 1];
    binary_sort(&mut arr);
    println!("{:?}", arr);
    
}

// make method for sorting binary sort
fn binary_sort(arr: &mut [i32]) {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;
    while left < right {
        let mut i = left;
        while i < right {
            if arr[i as usize] > arr[(i + 1) as usize] {
                arr.swap(i as usize, (i + 1) as usize);
            }
            i += 1;
        }
        right -= 1;
        let mut j = right;
        while j > left {
            if arr[j as usize] < arr[(j - 1) as usize] {
                arr.swap(j as usize, (j - 1) as usize);
            }
            j -= 1;
        }
        left += 1;
    }
}

#[test]
fn test_binary_sort() {
    let mut arr: [i32; 5] = [5, 3, 2, 4, 1];
    binary_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
fn test_optional() {
    let  name: &str = "Rust";
    
    if name == "Rust" {
        println!("Hello, Rust!");
    } else if name == "Go" {
        println!("Hello, Go!");
    } else {
        println!("Hello, World!");
    }
}

#[test]
fn test_array_assosiation() {
    let arr = [1,3,4,6];
    let mut sum = 0;
    for i in &arr {
        sum += i;
    }
    assert_eq!(sum, 14);
    
}

#[test]
fn test_map() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    log::info!("map: {:?}", map);
    map.insert("name", "Rust");
    map.insert("age", "5");
    map.insert("lang", "Rust");
    assert_eq!(map.get("name"), Some(&"Rust"));
    assert_eq!(map.get("age"), Some(&"5"));
    assert_eq!(map.get("lang"), Some(&"Rust"));
}



fn mark_class(mark: i32) -> String {
    if mark >=80 && mark <= 100 {
        "A".to_owned()
    } else if mark >= 60 && mark < 80 {
        "B".to_owned()
    } else if mark >= 40 && mark < 60 {
        "C".to_owned()
    } else {
        "D".to_owned()
    }
    
}

#[test]
fn test_mark_class() {
    assert_eq!(mark_class(90), "A");
    assert_eq!(mark_class(70), "B");
    assert_eq!(mark_class(50), "C");
    assert_eq!(mark_class(30), "D");
}



fn star(row: i32) {
    for i in 0..row {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}

#[test]
fn test_star() {
    star(5);
}

fn star_reverse(row: i32) {
    for i in 0..row {
        for _ in 0..row - i {
            print!("*");
        }
        println!();
    }
}

#[test]
fn test_star_reverse() {
    star_reverse(5);
}

fn star_pyramid(row: i32) {
    for i in 0..row {
        for _ in 0..row - i {
            print!(" ");
        }
        for _ in 0..2 * i + 1 {
            print!("*");
        }
        println!();
    }
}

#[test]
fn test_star_pyramid() {
    star_pyramid(5);
}

#[test]
fn test_star_pyramid_reverse() {
    star_pyramid_reverse(5);
}

fn star_pyramid_reverse(row: i32) {
    for i in 0..row {
        for _ in 0..i {
            print!(" ");
        }
        for _ in 0..2 * (row - i) - 1 {
            print!("*");
        }
        println!();
    }
}

#[test]
fn test_variable_shadowing() {
    let name = "Rust";
    println!("name: {}", name);
    let name = "Go";
    println!("name: {}", name);
    assert_eq!(name, "Go");
}

#[test]
fn test_variable_augmented() {
    let mut num = 90;
    println!("num: {}", num);
    num *= 100;
    println!("num: {}", num);
}

#[test]
fn test_boolean() {
    let a: bool = false;
    print!("a: {}", a);
}

#[test]
fn test_itteration() {
    for i in 0..5 {
        println!("i: {}", i);
    }
}

fn get_name() -> String {
    let name = String::from("Rust");
    return name;
}

#[test]
fn test_get_name() {
    let name = get_name();
    assert_eq!(name, "Rust");
}