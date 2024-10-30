use std::collections::HashMap;

// checks if a file is valid
pub fn check_valid_list(new: &Vec<&str>, old: &Vec<&str>) -> bool
{
    let mut new: Vec<String> = new.iter().map(|s| s.to_uppercase()).collect();
    let mut old: Vec<String> = old.iter().map(|s| s.to_uppercase()).collect();

    new.sort();
    old.sort();

    // using binary search to see if new list is a subset of old one
    for element in &new
    {
        if !old.binary_search(element).is_ok()
        {
            print!("not found: {}", element);
            return false;
        }
    }

    //checks that no values are repeated
    let mut element_count: HashMap<String, i32> = HashMap::new();

    for element in &new
    {
        let x = element_count.entry(element.to_string()).or_insert(0);
        *x += 1;
        if *x > 1
        {
            print!("repeated: {}", element);
            return false;
        }
    }
    return true;
}