#[derive(thiserror::Error, Debug)]
pub enum TreeChartError {
    #[error("The width should be set to something other than 0")]
    WidthUnset,

    #[error("Number of elements indivisible by width")]
    DimensionError,
}

pub struct TreeChartBuilder {
    chart: Vec<bool>,
    width: usize,
}

impl TreeChartBuilder {
    pub fn new() -> Self {
        TreeChartBuilder {
            chart: Vec::new(),
            width: 0,
        }
    }

    pub fn append(&mut self, tree_present: bool) {
        self.chart.push(tree_present);
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn build(self) -> Result<TreeChart, TreeChartError> {
        if self.width == 0 {
            return Err(TreeChartError::WidthUnset);
        }

        if self.chart.len() % self.width != 0 {
            return Err(TreeChartError::DimensionError);
        }

        Ok(TreeChart {
            chart: self.chart,
            width: self.width,
        })
    }
}

pub struct TreeChart {
    chart: Vec<bool>,
    width: usize,
}

impl TreeChart {
    pub fn get(&self, (x, y): (usize, usize)) -> Option<(bool, (usize, usize))> {
        //     ➡
        //     y
        // ⬇ x
        let y = y % self.width;

        let tree_present = self.chart.get(x * self.width + y).cloned();
        tree_present.map(|t| (t, (x, y)))
    }

    pub fn iter(&self, offset: (usize, usize)) -> TreeChartIterator {
        TreeChartIterator {
            offset,
            position: (0, 0),
            chart: self,
        }
    }
}

pub struct TreeChartIterator<'a> {
    offset: (usize, usize),
    position: (usize, usize),
    chart: &'a TreeChart,
}

impl<'a> Iterator for TreeChartIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((tree_present, (x, wrapped_y))) = self.chart.get(self.position) {
            self.position = (x + self.offset.0, wrapped_y + self.offset.1);
            Some(tree_present)
        } else {
            None
        }
    }
}

pub fn input(input: &str) -> Result<TreeChart, TreeChartError> {
    let mut builder = TreeChartBuilder::new();
    for (idx, line) in input.lines().enumerate() {
        let mut width = 0usize;
        for letter in line.chars() {
            builder.append(letter == '#');
            width += 1;
        }
        if idx == 0 {
            builder.set_width(width);
        }
    }

    builder.build()
}

pub fn count_trees_encountered(chart: &TreeChart, offset: (usize, usize)) -> usize {
    chart
        .iter(offset)
        .map(|tree_present| if tree_present { 1 } else { 0 })
        .sum()
}

pub fn part1(input: &TreeChart) {
    let tree_amt: usize = count_trees_encountered(input, (1, 3));
    println!("Trees encountered: {}", tree_amt);
}

pub fn part2(input: &TreeChart) {
    let answer: usize = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|offset| count_trees_encountered(input, *offset))
        .product();
    println!("Answer: {}", answer);
}
