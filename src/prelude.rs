use std::str::Lines;

pub struct ChunkLineIterator<'a> {
    contents: Lines<'a>,
    chunk_size: usize,
}

impl<'a> ChunkLineIterator<'a> {
    pub fn new(input: &'a str, chunk_size: usize) -> Self {
        Self {
            contents: input.lines(),
            chunk_size,
        }
    }
}

impl<'a> Iterator for ChunkLineIterator<'a> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        (0..self.chunk_size)
            .map(|_| self.contents.next().map(|c| c.to_string()))
            .collect()
    }
}
