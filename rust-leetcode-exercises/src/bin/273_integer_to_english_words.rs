struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let big_string = ["Thousand", "Million", "Billion"];
        let mut num = num;
        let mut parts = Vec::new();

        // Process groups of three digits from lowest to highest
        parts.push(Solution::number_to_words_helper(num % 1000));
        num /= 1000;

        for big in &big_string {
            if num == 0 { break; }
            if num % 1000 > 0 {
                let mut segment = Solution::number_to_words_helper(num % 1000);
                if !segment.is_empty() {
                    segment.push(' ');
                    segment.push_str(big);
                    parts.push(segment);
                }
            }
            num /= 1000;
        }

        // Collect non-empty parts, reverse and join by space
        parts
            .into_iter()
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<String>>()
            .into_iter()
            .rev()
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn number_to_words_helper(num: i32) -> String {
        let digit_string = [
            "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ];
        let teen_string = [
            "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen",
            "Eighteen", "Nineteen",
        ];
        let ten_string = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        let mut words = Vec::new();
        let mut num = num;

        if num >= 100 {
            words.push(digit_string[(num / 100) as usize].to_string());
            words.push("Hundred".to_string());
            num %= 100;
        }

        if num >= 10 && num < 20 {
            words.push(teen_string[(num % 10) as usize].to_string());
        } else {
            if num >= 20 {
                words.push(ten_string[(num / 10) as usize].to_string());
                num %= 10;
            }
            if num > 0 {
                words.push(digit_string[num as usize].to_string());
            }
        }

        words.join(" ")
    }
}

fn main() {
    let nums = [
        0,
        5,
        13,
        123,
        1005,
        12345,
        1234567,
        1234567891,
    ];
    for num in nums {
        println!("{} -> {}", num, Solution::number_to_words(num));
    }
}
