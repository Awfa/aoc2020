pub enum RowPartition {
    Front,
    Back,
}

pub enum SeatPartition {
    Left,
    Right,
}

pub struct RowPartitionIterator<T>
where
    T: Iterator<Item = char>,
{
    characters: T,
}

impl<T> RowPartitionIterator<T>
where
    T: Iterator<Item = char>,
{
    fn new(characters: T) -> Self {
        RowPartitionIterator { characters }
    }
}

impl<T> Iterator for RowPartitionIterator<T>
where
    T: Iterator<Item = char>,
{
    type Item = RowPartition;

    fn next(&mut self) -> Option<Self::Item> {
        match self.characters.next() {
            Some(letter) => {
                if letter == 'F' {
                    Some(RowPartition::Front)
                } else if letter == 'B' {
                    Some(RowPartition::Back)
                } else {
                    panic!("Unexpected character {}", letter)
                }
            }
            None => None,
        }
    }
}

pub struct SeatPartitionIterator<T>
where
    T: Iterator<Item = char>,
{
    characters: T,
}

impl<T> SeatPartitionIterator<T>
where
    T: Iterator<Item = char>,
{
    fn new(characters: T) -> Self {
        SeatPartitionIterator { characters }
    }
}

impl<T> Iterator for SeatPartitionIterator<T>
where
    T: Iterator<Item = char>,
{
    type Item = SeatPartition;

    fn next(&mut self) -> Option<Self::Item> {
        match self.characters.next() {
            Some(letter) => {
                if letter == 'L' {
                    Some(SeatPartition::Left)
                } else if letter == 'R' {
                    Some(SeatPartition::Right)
                } else {
                    panic!("Unexpected character {}", letter)
                }
            }
            None => None,
        }
    }
}

struct PartitionRange {
    low: usize,
    hi: usize,
}

impl PartitionRange {
    // Makes a range between [low, hi] where low is inclusive, and hi is inclusive
    pub fn new(low: usize, hi: usize) -> Self {
        PartitionRange { low, hi }
    }

    /// If the range has converged, returns the converged value - otherwise returns None
    pub fn upper_half(&mut self) -> Option<usize> {
        self.low = self.low + (self.hi - self.low) / 2 + 1;
        if self.low == self.hi {
            Some(self.low)
        } else {
            None
        }
    }

    /// If the range has converged, returns the converged value - otherwise returns None
    pub fn lower_half(&mut self) -> Option<usize> {
        self.hi = self.low + (self.hi - self.low) / 2;
        if self.low == self.hi {
            Some(self.low)
        } else {
            None
        }
    }

    pub fn get_converged_value(&self) -> Option<usize> {
        if self.low == self.hi {
            Some(self.low)
        } else {
            None
        }
    }
}

pub fn get_seat_id(boarding_pass: &str) -> usize {
    let mut row = PartitionRange::new(0, 127);
    for row_partition in RowPartitionIterator::new(boarding_pass.chars().take(7)) {
        match row_partition {
            RowPartition::Front => row.lower_half(),
            RowPartition::Back => row.upper_half(),
        };
    }
    let row = row.get_converged_value().unwrap();

    let mut col = PartitionRange::new(0, 7);
    for seat_partition in SeatPartitionIterator::new(boarding_pass.chars().skip(7).take(3)) {
        match seat_partition {
            SeatPartition::Left => col.lower_half(),
            SeatPartition::Right => col.upper_half(),
        };
    }
    let col = col.get_converged_value().unwrap();

    let seat_id = row * 8 + col;
    seat_id
}

pub fn part1(input: &str) {
    let highest_seat_id = input.lines().map(get_seat_id).max().unwrap();

    println!("{}", highest_seat_id);
}

pub fn part2(input: &str) {
    let mut seat_ids = input.lines().map(get_seat_id).collect::<Vec<_>>();
    seat_ids.sort();

    let seat_id = seat_ids
        .windows(2)
        .find(|window| window[0] + 2 == window[1])
        .map(|window| window[1] - 1);

    println!("Seat id = {:?}", seat_id);
}

#[cfg(test)]
mod test {
    use super::PartitionRange;
    #[test]
    fn range_test() {
        let mut range = PartitionRange::new(0, 127);
        assert_eq!(range.lower_half(), None);
        assert_eq!(range.low, 0);
        assert_eq!(range.hi, 63);

        assert_eq!(range.upper_half(), None);
        assert_eq!(range.low, 32);
        assert_eq!(range.hi, 63);

        assert_eq!(range.lower_half(), None);
        assert_eq!(range.low, 32);
        assert_eq!(range.hi, 47);

        assert_eq!(range.upper_half(), None);
        assert_eq!(range.low, 40);
        assert_eq!(range.hi, 47);

        assert_eq!(range.upper_half(), None);
        assert_eq!(range.low, 44);
        assert_eq!(range.hi, 47);

        assert_eq!(range.lower_half(), None);
        assert_eq!(range.low, 44);
        assert_eq!(range.hi, 45);

        assert_eq!(range.lower_half(), Some(44));
        assert_eq!(range.low, 44);
        assert_eq!(range.hi, 44);
    }
}
