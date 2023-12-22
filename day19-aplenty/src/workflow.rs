use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Part {
    pub(crate) x: u64,
    pub(crate) m: u64,
    pub(crate) a: u64,
    pub(crate) s: u64,
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct Condition {
    pub(crate) operand1: char,
    pub(crate) operand2: u64,
    pub(crate) operator: char,
}
#[derive(Debug, Clone)]
pub(crate) struct Rule {
    pub(crate) condition: Option<Condition>,
    pub(crate) destination: String,
}
#[derive(Debug, Clone)]
pub(crate) struct PartRange {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}
impl PartRange {
    pub(crate) fn permutations(&self) -> u64 {
        (*self.x.end() - *self.x.start() + 1)
            * (*self.m.end() - *self.m.start() + 1)
            * (*self.a.end() - *self.a.start() + 1)
            * (*self.s.end() - *self.s.start() + 1)
    }
    pub(crate) fn split_range(self, value: u64, attr: char) -> (Self, Self) {
        let attr_range = match attr {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => unreachable!(),
        };
        let (left, right) = (*attr_range.start()..=value, (value + 1)..=*attr_range.end());
        let mut lower_bound = self.clone();
        let mut upper_bound = self.clone();
        match attr {
            'x' => {
                lower_bound.x = left;
                upper_bound.x = right;
            }
            'm' => {
                lower_bound.m = left;
                upper_bound.m = right;
            }
            'a' => {
                lower_bound.a = left;
                upper_bound.a = right;
            }
            's' => {
                lower_bound.s = left;
                upper_bound.s = right;
            }
            _ => unreachable!(),
        }
        (lower_bound, upper_bound)
    }
}
impl Default for PartRange {
    fn default() -> Self {
        PartRange {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}
