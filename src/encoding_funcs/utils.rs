// TODO: use Generics T
pub fn get_vec_range_str(vec: &Vec<&str>, start: usize, end: usize) -> String {
    let mut output = String::new();
    for i in start..end {
        output = output + vec[i];
    }

    return output;
}
