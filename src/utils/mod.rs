use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// Load the raw string contents from one of the puzzle input files
pub fn load_data(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("data/{}", name));

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    contents
}

pub fn non_empty_lines(input: String) -> Vec<String> {
    input.split("\n")
        .into_iter()
        .filter(|e| e != &"")
        .map(|e| e.to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_non_empty_lines() {
        let input = "a\nb\n\nc\n".to_owned();
        
        let parts = non_empty_lines(input);
        let expected: Vec<String> = vec![
            String::from("a"),
            String::from("b"),
            String::from("c")
        ];
        
        assert_eq!(parts, expected);
    }
}
