/*
  Problem 33: String Compression

  Write a function that performs basic string compression using the counts of repeated characters.
  For example, "aabcccccaaa" becomes "a2b1c5a3".
  If the compressed string is not shorter than the original, return the original string.

  Run the tests for this problem with:
    cargo test --test string_compression_test
*/

pub fn compress(s: &str) -> String {
    let mut compressed = String::new();
    let mut chars = s.chars();

    let mut current = chars.next().unwrap();
    let mut count = 1;

    for ch in chars {
        if ch == current {
            count += 1;
        } else {
            compressed.push(current);
            compressed.push_str(&count.to_string());

            current = ch;
            count = 1;
        }
    }

    compressed.push(current);
    compressed.push_str(&count.to_string());

    if compressed.len() < s.len() {
        compressed
    } else {
        s.to_string()
    }
}
