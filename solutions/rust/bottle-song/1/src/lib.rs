const ONES: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
    "seventeen", "eighteen", "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: [(u32, &str); 4] = [
    (1_000_000_000, "billion"),
    (1_000_000, "million"),
    (1_000, "thousand"),
    (100, "hundred"),
];

fn encode(no: u32) -> String {
    match no {
        0 => "no".to_string(),
        1..=19 => ONES[no as usize].to_string(),
        20..=99 => {
            let tens = (no / 10) as usize;
            let ones = (no % 10) as usize;
            if ones == 0 {
                TENS[tens].to_string()
            } else {
                format!("{}-{}", TENS[tens], ONES[ones])
            }
        }
        _ => {
            let &(scale, name) = SCALES.iter().find(|&&(x, _)| no >= x).unwrap();
            let high = encode(no / scale);
            let rem = no % scale;
            if rem == 0 {
                format!("{} {}", high, name)
            } else {
                format!("{} {} {}", high, name, encode(rem))
            }
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn bottle_word(count: u32) -> &'static str {
    if count == 1 { "bottle" } else { "bottles" }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    for i in (start_bottles - take_down + 1..=start_bottles).rev() {
        let current = capitalize(&encode(i));
        let next = encode(i - 1);
        let current_word = bottle_word(i);
        let next_word = bottle_word(i - 1);

        verses.push(format!(
            "{current} green {current_word} hanging on the wall,\n\
             {current} green {current_word} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {next} green {next_word} hanging on the wall."
        ));
    }

    verses.join("\n\n")
}
