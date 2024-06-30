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