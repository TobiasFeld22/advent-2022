#[derive(Debug, Copy, Clone)]
pub struct Section {
    start: i32,
    end: i32,
}

impl Section {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
    pub fn parse(str: &str) -> Self {
        let mut split = str.split("-");

        let start = split.next().unwrap().parse::<i32>().unwrap();
        let end = split.next().unwrap().parse::<i32>().unwrap();
        Self { start, end }
    }

    pub fn overlaps(&self, other: Section) -> bool {
        return self.start.max(other.start) <= self.end.min(other.end);
    }
}
