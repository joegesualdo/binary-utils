use std::num::ParseIntError;

fn split_string_with_spaces_for_substrings_with_length(s: &str, length: u64) -> String {
    let string_with_spaces_seperating_substrings =
        s.chars().enumerate().fold(String::new(), |acc, (i, c)| {
            //if i != 0 && i == 11 {
            if i != 0 && (i % length as usize == 0) {
                format!("{} {}", acc, c)
            } else {
                format!("{}{}", acc, c)
            }
        });
    string_with_spaces_seperating_substrings
}

pub fn decode_binary(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(9)
        .map(|i| u8::from_str_radix(&s[i..i + 8], 2))
        .collect()
}

pub fn split_binary_string_into_framents_of_11_bits(binary_string: &str) -> Vec<String> {
    let entropy_plus_checksum_binary_with_spaces_seperating =
        split_string_with_spaces_for_substrings_with_length(&binary_string, 11);
    let word_binary: Vec<&str> = entropy_plus_checksum_binary_with_spaces_seperating
        .split(" ")
        .collect();
    word_binary.iter().map(|&s| s.to_string()).collect()
}
pub fn split_binary_string_into_framents_of_5_bits(binary_string: &str) -> Vec<String> {
    let entropy_plus_checksum_binary_with_spaces_seperating =
        split_string_with_spaces_for_substrings_with_length(&binary_string, 5);
    let word_binary: Vec<&str> = entropy_plus_checksum_binary_with_spaces_seperating
        .split(" ")
        .collect();
    word_binary.iter().map(|&s| s.to_string()).collect()
}

fn convert_binary_to_int(binary_string: &str) -> isize {
    let bin_idx = binary_string;
    let intval = isize::from_str_radix(bin_idx, 2).unwrap();
    intval
}
pub fn convert_to_binary_string(num: u8, bits_to_show_count: u64) -> String {
    fn crop_letters(s: &str, pos: usize) -> &str {
        match s.char_indices().skip(pos).next() {
            Some((pos, _)) => &s[pos..],
            None => "",
        }
    }
    fn format_binary_with_4_bits(num: u8) -> String {
        // The 06 pads with zeros to a width of 6. That width includes 0b (length=2)
        format!("{:#06b}", num)
    }
    fn format_binary_with_8_bits(num: u8) -> String {
        // The 10 pads with zeros to a width of 10. That width includes 0b (length=2)
        format!("{:#010b}", num)
    }
    let binary_string_with_prefix = match bits_to_show_count {
        4 => format_binary_with_4_bits(num),
        8 => format_binary_with_8_bits(num),
        _ => panic!(
            "binary_string_without_prefix: bits_to_show_count of {} not supported",
            bits_to_show_count
        ),
    };
    let binary_string_without_prefix = crop_letters(&binary_string_with_prefix, 2);
    binary_string_without_prefix.to_string()
}

pub fn get_binary_string_for_byte_array(byte_array: &Vec<u8>) -> String {
    let mut binary_string = String::new();
    for i in byte_array {
        let binary_str = convert_to_binary_string(*i, 8);
        binary_string.push_str(binary_str.as_str())
    }
    binary_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
