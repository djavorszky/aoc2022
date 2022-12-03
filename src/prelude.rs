use std::str::Lines;

pub struct ConstChunkIterator<'a, const N: usize> {
    contents: Lines<'a>,
    chunk_size: usize,
}

impl<'a, const N: usize> ConstChunkIterator<'a, N> {
    pub fn new(input: &'a str) -> Self {
        Self {
            contents: input.lines(),
            chunk_size: N,
        }
    }
}

impl<'a, const N: usize> Iterator for ConstChunkIterator<'a, N> {
    type Item = [&'a str; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = [""; N];

        (0..self.chunk_size).enumerate().for_each(|(idx, _)| {
            result[idx] = self.contents.next().unwrap_or_default();
        });

        if (result.iter().any(|f| *f == "")) {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a, const N: usize> From<&'a str> for ConstChunkIterator<'a, N> {
    fn from(s: &'a str) -> Self {
        Self::new(s)
    }
}
