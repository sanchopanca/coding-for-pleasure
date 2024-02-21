use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for s in strs {
        let mut sorted = s.chars().collect::<Vec<_>>();
        sorted.sort();

        map.entry(sorted).or_default().push(s);
    }
    let mut result = Vec::new();

    for (_, mut value) in map {
        value.sort();
        result.push(value);
    }

    result
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::group_anagrams;

    #[test]
    fn test_group_anagrams() {
        let result = group_anagrams(
            ["eat", "tea", "tan", "ate", "nat", "bat"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        );

        assert_eq!(result.len(), 3);
        assert!(result.contains(&vec!["bat".to_string()]));
        assert!(result.contains(&vec!["nat".to_string(), "tan".to_string()]));
        assert!(result.contains(&vec![
            "ate".to_string(),
            "eat".to_string(),
            "tea".to_string()
        ]));

        assert_eq!(
            group_anagrams(vec!["".to_string()]),
            vec![vec!["".to_string()]]
        );

        assert_eq!(
            group_anagrams(vec!["a".to_string()]),
            vec![vec!["a".to_string()]]
        );
    }
}
