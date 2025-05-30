use std::{
    cmp::Ordering,
    fmt,
    iter::{self, repeat_n},
    ops, slice,
    str::FromStr,
};

use foldhash::HashSet;
use regex::bytes::Regex;
use serde::de::{Deserialize, Deserializer, Error};

#[derive(Clone)]
pub struct SelectColumns {
    selectors: Vec<Selector>,
    invert:    bool,
}

impl SelectColumns {
    pub fn parse(mut s: &str) -> Result<SelectColumns, String> {
        let is_empty = s.is_empty();
        let bytes = s.as_bytes();
        let invert = if !is_empty && bytes[0] == b'!' {
            s = &s[1..];
            true
        } else {
            false
        };
        Ok(SelectColumns {
            selectors: SelectorParser::new(s).parse()?,
            invert,
        })
    }

    pub fn selection(
        &self,
        first_record: &csv::ByteRecord,
        use_names: bool,
    ) -> Result<Selection, String> {
        if self.selectors.is_empty() {
            return Ok(Selection(if self.invert {
                // Inverting everything means we get nothing.
                vec![]
            } else {
                (0..first_record.len()).collect()
            }));
        }

        let mut map = vec![];
        for sel in &self.selectors {
            let idxs = sel.indices(first_record, use_names);
            map.extend(idxs?);
        }
        if self.invert {
            let set: HashSet<_> = map.into_iter().collect();
            let mut map = vec![];
            for i in 0..first_record.len() {
                if !set.contains(&i) {
                    map.push(i);
                }
            }
            return Ok(Selection(map));
        }
        Ok(Selection(map))
    }

    #[allow(dead_code)]
    pub const fn is_empty(&self) -> bool {
        self.selectors.is_empty()
    }
}

impl fmt::Debug for SelectColumns {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.selectors.is_empty() {
            write!(f, "<All>")
        } else {
            let strs: Vec<_> = self
                .selectors
                .iter()
                .map(|sel| format!("{sel:?}"))
                .collect();
            write!(f, "{}", strs.join(", "))
        }
    }
}

impl<'de> Deserialize<'de> for SelectColumns {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<SelectColumns, D::Error> {
        let raw = String::deserialize(d)?;
        SelectColumns::parse(&raw).map_err(D::Error::custom)
    }
}

struct SelectorParser {
    chars: Vec<char>,
    pos:   usize,
}

impl SelectorParser {
    fn new(s: &str) -> SelectorParser {
        SelectorParser {
            chars: s.chars().collect(),
            pos:   0,
        }
    }

    fn parse(&mut self) -> Result<Vec<Selector>, String> {
        if (self.chars.first(), self.chars.last()) == (Some(&'/'), Some(&'/')) {
            if self.chars.len() == 2 {
                return fail_format!("Empty regex: {}", self.chars.iter().collect::<String>());
            }
            let re: String = self.chars[1..(self.chars.len() - 1)].iter().collect();
            let Ok(regex) = Regex::new(&re) else {
                return fail_format!("Invalid regex: {re}");
            };
            return Ok(vec![Selector::Regex(regex)]);
        }

        let mut sels = vec![];
        loop {
            if self.cur().is_none() {
                break;
            }
            let f1: OneSelector = if self.cur() == Some('-') {
                OneSelector::Start
            } else {
                self.parse_one()?
            };
            let f2: Option<OneSelector> = if self.cur() == Some('-') {
                self.bump();
                Some(if self.is_end_of_selector() {
                    OneSelector::End
                } else {
                    self.parse_one()?
                })
            } else {
                None
            };
            if !self.is_end_of_selector() {
                return fail_format!(
                    "Expected end of field but got '{}' instead.",
                    self.cur().unwrap()
                );
            }
            sels.push(match f2 {
                Some(end) => Selector::Range(f1, end),
                None => Selector::One(f1),
            });
            self.bump();
        }
        Ok(sels)
    }

    fn parse_one(&mut self) -> Result<OneSelector, String> {
        let name = if self.cur() == Some('"') {
            self.bump();
            self.parse_quoted_name()?
        } else {
            if self.cur() == Some('_') {
                self.bump();
                return Ok(OneSelector::End);
            }
            self.parse_name()
        };
        Ok(if self.cur() == Some('[') {
            let idx = self.parse_index()?;
            OneSelector::IndexedName(name, idx)
        } else {
            match FromStr::from_str(&name) {
                Err(_) => OneSelector::IndexedName(name, 0),
                Ok(idx) => OneSelector::Index(idx),
            }
        })
    }

    fn parse_name(&mut self) -> String {
        let mut name = String::new();
        loop {
            if self.is_end_of_field() || self.cur() == Some('[') {
                break;
            }
            name.push(self.cur().unwrap());
            self.bump();
        }
        name
    }

    fn parse_quoted_name(&mut self) -> Result<String, String> {
        let mut name = String::new();
        loop {
            match self.cur() {
                None => {
                    return fail!("Unclosed quote, missing closing \".");
                },
                Some('"') => {
                    self.bump();
                    if self.cur() == Some('"') {
                        self.bump();
                        name.push('"');
                        name.push('"');
                        continue;
                    }
                    break;
                },
                Some(c) => {
                    name.push(c);
                    self.bump();
                },
            }
        }
        Ok(name)
    }

    fn parse_index(&mut self) -> Result<usize, String> {
        assert_eq!(self.cur().unwrap(), '[');
        self.bump();

        let mut idx = String::new();
        loop {
            match self.cur() {
                None => {
                    return fail!("Unclosed index bracket, missing closing ].");
                },
                Some(']') => {
                    self.bump();
                    break;
                },
                Some(c) => {
                    idx.push(c);
                    self.bump();
                },
            }
        }
        FromStr::from_str(&idx)
            .map_err(|err| format!("Could not convert '{idx}' to an integer: {err}"))
    }

    fn cur(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn is_end_of_field(&self) -> bool {
        self.cur().is_none_or(|c| c == ',' || c == '-')
    }

    fn is_end_of_selector(&self) -> bool {
        self.cur().is_none_or(|c| c == ',')
    }

    const fn bump(&mut self) {
        if self.pos < self.chars.len() {
            self.pos += 1;
        }
    }
}

#[derive(Clone)]
enum Selector {
    One(OneSelector),
    Range(OneSelector, OneSelector),
    Regex(Regex),
}

#[derive(Clone)]
enum OneSelector {
    Start,
    End,
    Index(usize),
    IndexedName(String, usize),
}

impl Selector {
    fn indices(
        &self,
        first_record: &csv::ByteRecord,
        use_names: bool,
    ) -> Result<Vec<usize>, String> {
        match *self {
            Selector::One(ref sel) => sel.index(first_record, use_names).map(|i| vec![i]),
            Selector::Range(ref sel1, ref sel2) => {
                let i1 = sel1.index(first_record, use_names)?;
                let i2 = sel2.index(first_record, use_names)?;
                Ok(match i1.cmp(&i2) {
                    Ordering::Equal => vec![i1],
                    Ordering::Less => (i1..=i2).collect(),
                    Ordering::Greater => {
                        let mut inds = vec![];
                        let mut i = i1 + 1;
                        while i > i2 {
                            i -= 1;
                            inds.push(i);
                        }
                        inds
                    },
                })
            },
            Selector::Regex(ref re) => {
                let inds: Vec<usize> = first_record
                    .iter()
                    .enumerate()
                    .filter(|(_, h)| re.is_match(h))
                    .map(|(i, _)| i)
                    .collect();
                if inds.is_empty() {
                    return fail_format!(
                        "Selector regex '{re}' does not match any columns in the CSV header."
                    );
                }
                Ok(inds)
            },
        }
    }
}

impl OneSelector {
    fn index(&self, first_record: &csv::ByteRecord, use_names: bool) -> Result<usize, String> {
        match *self {
            OneSelector::Start => Ok(0),
            OneSelector::End => Ok(if first_record.is_empty() {
                0
            } else {
                first_record.len() - 1
            }),
            OneSelector::Index(i) => {
                if first_record.is_empty() {
                    return fail!("Input is empty.");
                }
                if i < 1 || i > first_record.len() {
                    fail_format!(
                        "Selector index {i} is out of bounds. Index must be >= 1 and <= {}.",
                        first_record.len()
                    )
                } else {
                    // Indices given by user are 1-offset. Convert them here!
                    Ok(i - 1)
                }
            },
            OneSelector::IndexedName(ref s, sidx) => {
                if !use_names {
                    return fail_format!(
                        "Cannot use names ('{s}') in selection with --no-headers set."
                    );
                }
                let mut num_found = 0;
                for (i, field) in first_record.iter().enumerate() {
                    if field == s.as_bytes() {
                        if num_found == sidx {
                            return Ok(i);
                        }
                        num_found += 1;
                    }
                }
                if num_found == 0 {
                    fail_format!(
                        "Selector name '{s}' does not exist as a named header in the given CSV \
                         data."
                    )
                } else {
                    fail_format!(
                        "Selector index '{sidx}' for name '{s}' is out of bounds. Must be >= 0 \
                         and <= {}.",
                        num_found - 1
                    )
                }
            },
        }
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Selector::One(ref sel) => sel.fmt(f),
            Selector::Range(ref s, ref e) => write!(f, "Range({s:?}, {e:?})"),
            Selector::Regex(ref re) => re.fmt(f),
        }
    }
}

impl fmt::Debug for OneSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OneSelector::Start => write!(f, "Start"),
            OneSelector::End => write!(f, "End"),
            OneSelector::Index(idx) => write!(f, "Index({idx})"),
            OneSelector::IndexedName(ref s, idx) => write!(f, "IndexedName({s}[{idx}])"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Selection(Vec<usize>);

type _GetField = for<'c> fn(&mut &'c csv::ByteRecord, &usize) -> Option<&'c [u8]>;

impl Selection {
    #[inline]
    /// Returns an iterator that yields selected fields from a CSV record.
    ///
    /// This method takes a CSV record and returns an iterator that yields only the fields
    /// specified by this Selection. The fields are returned in the order they were selected.
    ///
    /// # Arguments
    ///
    /// * `row` - The CSV record to select fields from
    ///
    /// # Returns
    ///
    /// An iterator that yields references to the selected fields as byte slices
    pub fn select<'a, 'b>(
        &'a self,
        row: &'b csv::ByteRecord,
    ) -> iter::Scan<slice::Iter<'a, usize>, &'b csv::ByteRecord, _GetField> {
        #[allow(clippy::trivially_copy_pass_by_ref)]
        fn get_field<'c>(row: &mut &'c csv::ByteRecord, idx: &usize) -> Option<&'c [u8]> {
            row.get(*idx)
        }

        let get_field: _GetField = get_field;
        self.iter().scan(row, get_field)
    }

    pub fn normal(&self) -> NormalSelection {
        let Selection(inds) = self;
        if inds.is_empty() {
            return NormalSelection(vec![]);
        }

        let mut normal = inds.clone();
        normal.sort_unstable();
        normal.dedup();
        let mut set: Vec<_> = repeat_n(false, normal[normal.len() - 1] + 1).collect();
        for i in normal {
            set[i] = true;
        }
        NormalSelection(set)
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }
}

impl ops::Deref for Selection {
    type Target = [usize];

    fn deref(&self) -> &[usize] {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct NormalSelection(Vec<bool>);

type _NormalScan<'a, T, I> = iter::Scan<iter::Enumerate<I>, &'a [bool], _NormalGetField<T>>;

type _NormalFilterMap<'a, T, I> =
    iter::FilterMap<_NormalScan<'a, T, I>, fn(Option<T>) -> Option<T>>;

type _NormalGetField<T> = fn(&mut &[bool], (usize, T)) -> Option<Option<T>>;

impl NormalSelection {
    /// Selects elements from an iterator based on the normal selection pattern.
    ///
    /// This method takes an iterator and returns a filtered version that only includes
    /// elements at positions marked as true in the selection pattern.
    ///
    /// # Arguments
    ///
    /// * `row` - An iterator containing elements to filter
    ///
    /// # Returns
    ///
    /// Returns a filtered iterator that only yields elements at selected positions
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of elements in the iterator
    /// * `I` - The type of the input iterator
    pub fn select<T, I>(&self, row: I) -> _NormalFilterMap<'_, T, I>
    where
        I: Iterator<Item = T>,
    {
        const fn filmap<T>(v: Option<T>) -> Option<T> {
            v
        }
        #[allow(clippy::option_option)]
        fn get_field<T>(set: &mut &[bool], t: (usize, T)) -> Option<Option<T>> {
            let (i, v) = t;
            if i < set.len() && set[i] {
                Some(Some(v))
            } else {
                Some(None)
            }
        }
        let get_field: _NormalGetField<T> = get_field;
        let filmap: fn(Option<T>) -> Option<T> = filmap;
        row.enumerate().scan(&**self, get_field).filter_map(filmap)
    }

    pub fn len(&self) -> usize {
        self.iter().filter(|b| **b).count()
    }
}

impl ops::Deref for NormalSelection {
    type Target = [bool];

    fn deref(&self) -> &[bool] {
        &self.0
    }
}
