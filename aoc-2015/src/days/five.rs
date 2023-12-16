const DISALLOWED_PAIRS: [&str; 4] = [
    "ab", "cd", "pq", "xy"
];

pub fn task_one(data: &str) -> i32 {
    let mut count = 0;

    for line in data.lines() {
        let mut vowel_count = 0;
        let mut contains_double = false;
        if DISALLOWED_PAIRS.iter().any(|&pair| line.contains(pair)) {
            continue;
        }

        for (i, char) in line.chars().enumerate() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&char) {
                vowel_count += 1;
            }
            if i < line.len() - 1 && char == line.chars().nth(i + 1).unwrap() {
                contains_double = true;
            }
        }

        if vowel_count >= 3 && contains_double {
            count += 1;
        }
    }
    count
}

#[test]
fn test_one() {
    let input = "zgsnvdmlfuplrubt
vlhagaovgqjmgvwq
ffumlmqwfcsyqpss
zztdcqzqddaazdjp
eavfzjajkjesnlsb
urrvucyrzzzooxhx
xdwduffwgcptfwad
orbryxwrmvkrsxsr
jzfeybjlgqikjcow
mayoqiswqqryvqdi
iiyrkoujhgpgkcvx
egcgupjkqwfiwsjl
zbgtglaqqolttgng";
    assert_eq!(task_one(input), 3);
}

pub fn task_two(data: &str) -> i32 {
    let mut count = 0;

    for line in data.lines() {
        let mut pair = false;
        let mut pairs: Vec<(usize, String)> = vec![];
        let mut repeat = false;
        for i in 0..line.len() - 1 {
            let chars = line.chars().collect::<Vec<_>>();
            let curr_pair = format!("{}{}", chars[i], chars[i + 1]);
            if pairs.iter().any(|(j, old_pair)| old_pair == &curr_pair && j != &i) {
                pair = true;
            } else {
                pairs.push((i + 1, curr_pair.clone()));
            }
        }

        for (i, char) in line.chars().enumerate() {
            if i < line.len() - 2 {
                let new_char = line.chars().collect::<Vec<_>>()[i + 2];
                if new_char == char {
                    repeat = true;
                }
            }
        }

        if repeat && pair {
            count += 1;
        }
    }
    count
}

#[test]
fn test_two() {
    let input = "zgsnvdmlfuplrubt
vlhagaovgqjmgvwq
ffumlmqwfcsyqpss
zztdcqzqddaazdjp
eavfzjajkjesnlsb
urrvucyrzzzooxhx
xdwduffwgcptfwad
orbryxwrmvkrsxsr
jzfeybjlgqikjcow
mayoqiswqqryvqdi
iiyrkoujhgpgkcvx
egcgupjkqwfiwsjl
zbgtglaqqolttgng";
    assert_eq!(task_two(input), 2);
}
