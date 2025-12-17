use crate::textarea::text::Text;

impl Text {
    /// Splits this `Text` into a vector of `Text` segments, using `substring` as the delimiter.
    pub fn split(&self, substring: &str) -> Vec<Text> {
        if substring.is_empty() {
            return vec![self.clone()];
        }

        let pattern: Vec<char> = substring.chars().collect();
        let pattern_length = pattern.len();

        let mut result = Vec::new();

        let mut segment_start_index = 0;
        let mut search_index = 0;

        while search_index + pattern_length <= self.chars.len() {
            let matches_pattern = self.chars[search_index..search_index + pattern_length]
                .iter()
                .map(|character| character.value)
                .eq(pattern.iter().copied());

            if matches_pattern {
                result.push(Text {
                    chars: self.chars[segment_start_index..search_index].to_vec(),
                });

                search_index += pattern_length;
                segment_start_index = search_index;
            } else {
                search_index += 1;
            }
        }

        // Push remaining tail
        result.push(Text {
            chars: self.chars[segment_start_index..].to_vec(),
        });

        result
    }
}
