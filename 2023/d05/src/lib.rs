#![feature(iterator_try_collect)]

use core::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Almanac {
    /// Seeds
    to_plant: Vec<usize>,

    maps: HashMap<Kind, (Kind, Vec<AlmanacMapping>)>,
}

impl Almanac {
    /// # Errors
    /// - If the chain of types in the stored mappings has a dead end and does not eventually resolve to the target type
    /// - This will actually also infinitely recur if used on a looping structure so... dont do that.
    pub fn map_find_min(
        &self,
        kind: &Kind,
        val: usize,
        len: usize,
        target_kind: &Kind,
    ) -> Result<(usize, Kind), &'static str> {
        const NOP_MAP: AlmanacMapping = AlmanacMapping {
            len: 0,
            source_start: usize::MAX,
            destination_start: usize::MAX,
        };
        // stores the current ranges mapped. Reduce to find minimum at the end
        // Initialize to simply one range, the input
        let mut current_kind = kind;
        let mut current_range = vec![val..(val + len)];

        // Loop through mappings until kind matches target
        while current_kind != target_kind {
            if let Some((new_kind, maps)) = self.maps.get(current_kind) {
                let mut new_ranges = vec![];

                current_kind = new_kind;
                let mut old_ranges = current_range.into_iter();
                let mut maps = maps.iter();

                let map: Option<&AlmanacMapping> = maps.next();

                while let Some(src) = old_ranges.next() {
                    if src.start < map.unwrap_or(&NOP_MAP).source_start {
                        let subsegment_length =
                            if src.contains(&map.unwrap_or(&NOP_MAP).source_start) {
                                map.unwrap_or(&NOP_MAP).source_start - src.start
                            } else {
                                src.count()
                            };

                        // Push unmapped section
                        new_ranges.push((src.start, subsegment_length));
                    }
                }
                // Ranges should all be mapped
                current_range = new_ranges;
            } else {
                break;
            }
        }
        if current_kind == target_kind {
            unimplemented!()
        } else {
            Err("Could not resolve type conversions")
        }
    }

    /// # Errors
    /// - Malformed input coult make this iterator empty
    pub fn min_seed_loc(&self) -> Result<usize, &str> {
        let temp: Result<Vec<(usize, Kind)>, _> = self
            .to_plant
            .iter()
            .map(|s| self.map_find_min(&Kind::Seed, *s, 1, &Kind::Location))
            .collect();
        temp?.iter().map(|v| v.0).min().ok_or("Empty iter??")
    }

    /// Largely the same as the last one, reinterpret seeds differently though
    /// # Errors
    ///
    /// # Panics
    pub fn min_seed_loc_v2(&self) -> Result<usize, &str> {
        let mut seeds = self.to_plant.iter();

        let mut min_location = usize::MAX;

        while let Some(seed) = seeds.next() {
            let length = seeds.next().ok_or("odd no. of seeds?")?;
            dbg!(seed);

            let (location, _) = self.map_find_min(&Kind::Seed, *seed, *length, &Kind::Location)?;

            if location < min_location {
                min_location = location;
            }
        }

        Ok(min_location)
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Kind {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Kind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),

            _ => Err("Didn't find kind"),
        }
    }
}

#[derive(Debug)]
pub struct AlmanacMapping {
    len: usize,
    source_start: usize,
    destination_start: usize,
}

impl AlmanacMapping {
    /// # Panics
    /// - If type conversions overflow
    #[must_use]
    #[allow(clippy::unwrap_used)]
    pub fn try_map(&self, val: usize) -> Option<usize> {
        let range = self.source_start..(self.source_start + self.len);

        let offset: i64 = i64::try_from(self.destination_start).unwrap()
            - i64::try_from(self.source_start).unwrap();

        if range.contains(&val) {
            Some(usize::try_from(i64::try_from(val).unwrap() + offset).unwrap())
        } else {
            None
        }
    }
}

impl FromStr for AlmanacMapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace().map(str::parse);
        let destination_start = nums
            .next()
            .ok_or("out of numbers")?
            .map_err(|_| "int parse error")?;

        let source_start = nums
            .next()
            .ok_or("out of numbers")?
            .map_err(|_| "int parse error")?;

        let len = nums
            .next()
            .ok_or("out of numbers")?
            .map_err(|_| "int parse error")?;

        Ok(Self {
            len,
            source_start,
            destination_start,
        })
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.split("\n\n");
        let mut seeds_to_plant = None;

        let mut maps = HashMap::new();

        for c in chunks {
            let mut i = c.split(':');
            let key = i.next().ok_or(format!("no key in {c}"))?;
            let val = i.next().ok_or(format!("no val in {c}"))?;

            if let "seeds" = key {
                let seeds: Result<Vec<usize>, _> = val.split_whitespace().map(str::parse).collect();
                seeds_to_plant
                    .replace(seeds.map_err(|_| format!("Failed to parse ints in {val}"))?);
            } else {
                let kinds = key
                    .split_whitespace()
                    .next()
                    .ok_or(format!("Key parse error from {key}"))?;

                let mut t = kinds.split('-');
                let lhs: Kind = t.next().ok_or("no src kind")?.parse()?;
                let _ = t.next().ok_or("empty iter")?;
                let rhs: Kind = t.next().ok_or("no src kind")?.parse()?;

                let ranges: Result<Vec<AlmanacMapping>, _> = val
                    .lines()
                    .filter(|s| !s.is_empty())
                    .map(str::parse)
                    .collect();
                let mut ranges = ranges?;

                // Necessary for later step
                ranges.sort_by_key(|r| r.source_start);

                maps.insert(lhs, (rhs, ranges));
            }
        }

        Ok(Self {
            to_plant: seeds_to_plant.ok_or("Did not find seeds")?,
            maps,
        })
    }
}
