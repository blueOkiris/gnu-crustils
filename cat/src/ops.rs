/*
 * Author: Dylan Turner
 * Description: Operations used in basename
 */

// Perform basename op on str. If suff None, rm
pub fn perform_basename(name: &String, suffix: &Option<String>, use_null: bool) -> String {
    let mut base = basename(name);
    if suffix.is_some() {
        base = remove_suffix(&base, &suffix.clone().unwrap());
    }
    if !use_null {
        base += "\n";
    }
    base
}

// Remove suffix from the end of a string unless it is entirely the suffix
fn remove_suffix(name: &String, suffix: &String) -> String {
    let mut new_name = name.clone();

    if !name.ends_with(suffix) || name.len() == suffix.len() {
        return new_name;
    }

    for _ in 0..suffix.len() {
        new_name.remove(new_name.len() - 1);
    }
    new_name
}

// Actual base_name lib code (from lib/ in coreutils)
fn basename(name: &String) -> String {
    let mut ends_in_slash = false;
    let mut base_str = name.clone();
    while base_str.ends_with("/") {
        ends_in_slash = true;
        base_str.pop();
    }
    base_str = match base_str.rfind("/") {
        None => base_str,
        Some(last_slash) => String::from(base_str.split_at(last_slash + 1).1)
    };
    if ends_in_slash {
        base_str += "/";
    }
    base_str
}
