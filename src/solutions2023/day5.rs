use crate::files::contents;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;
use std::cmp::min;
use std::fmt::{Debug, Display, Write};
use std::ops::Add;
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumCount, EnumIter};

type PuzzleResult = usize;

pub fn solution() -> (PuzzleResult, PuzzleResult) {
    inner(&contents("inputs/2023/day5.txt")).unwrap()
}

fn inner(input: &String) -> Result<(PuzzleResult, PuzzleResult)> {
    let mut part1 = usize::MAX;
    let sources_part1 = parse(input, false)?;
    for seed in sources_part1.seeds.iter() {
        let dest = sources_part1.map_seed(*seed)[Category::Location as usize];
        part1 = min(part1, dest);
    }

    let sources_part2 = parse(input, true)?;
    let part2 = sources_part2
        .seeds
        .par_iter()
        .map(|seed| {
            let dest = sources_part2.map_seed(*seed)[Category::Location as usize];
            dest
        })
        .min()
        .unwrap();

    Ok((part1, part2))
}

#[derive(Debug, Default, Display, EnumIter, EnumCount, Copy, Clone)]
enum Category {
    #[default]
    Soil = 0,
    Fertilizer = 1,
    Water = 2,
    Light = 3,
    Temperature = 4,
    Humidity = 5,
    Location = 6,
}

#[derive(Debug, Default, PartialEq)]
struct MappingRange {
    source: usize,
    destination: usize,
    size: usize,
}

impl MappingRange {
    fn new(source: usize, destination: usize, size: usize) -> MappingRange {
        MappingRange {
            source,
            destination,
            size,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    pub(crate) fn map(&self, source: usize) -> usize {
        for range in self.ranges.iter() {
            if source >= range.source && source < range.source + range.size {
                let distance = source - range.source;
                return range.destination + distance;
            }
        }
        source
    }
}

#[derive(Default)]
struct Sources {
    mappings: [Mapping; Category::COUNT],
    seeds: Vec<usize>,
}

impl Sources {
    pub(crate) fn map_seed(&self, seed: usize) -> [usize; Category::COUNT] {
        let mut source = seed;
        std::array::from_fn(|cat| {
            let mapping = &self.mappings[cat];
            source = mapping.map(source);
            source
        })
    }
}

fn parse(input: &String, seed_ranges: bool) -> Result<Sources> {
    let lines = input.lines().collect_vec();

    let seeds_components = lines[0]["seeds: ".len()..]
        .split_ascii_whitespace()
        .collect_vec();
    let seeds: Vec<usize> = if seed_ranges {
        seeds_components
            .chunks(2)
            .map(|chunk| {
                let (begin, size) = chunk
                    .iter()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                begin..begin + size
            })
            .flatten()
            .collect()
    } else {
        seeds_components
            .iter()
            .map(|num| num.parse::<usize>().unwrap())
            .collect()
    };

    let indices = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if line.is_empty() { Some(i + 2) } else { None })
        .collect_vec();
    assert_eq!(indices.len(), Category::COUNT);

    let mappings: [Mapping; Category::COUNT] = std::array::from_fn(|i| {
        let begin = indices[i];
        let end = if i >= indices.len() - 1 {
            lines.len()
        } else {
            indices[i + 1] - 2
        };
        let mut ranges = vec![];
        for i in begin..end {
            let elems = lines[i]
                .split_ascii_whitespace()
                .map(|el| {
                    el.parse::<usize>()
                        .expect(format!("Failed to parse element {el}").as_str())
                })
                .collect_vec();
            let mapping_range = MappingRange::new(elems[1], elems[0], elems[2]);
            ranges.push(mapping_range);
        }

        Mapping { ranges }
    });

    let ret = Sources { seeds, mappings };

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings::SkipEmptyLines;
    use itertools::Itertools;
    use map_macro::{hash_map, hash_set};
    use nofmt;
    use pretty_assertions::assert_eq as pretty_assert_eq;
    use textwrap::dedent;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    fn program() -> String {
        return dedent(
            "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4",
        )
        .skip_empty_start_lines();
    }
    #[test]
    fn test_part1_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.0;
        assert_eq!(actual, 35);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<()> {
        let input_string = program();
        let actual = inner(&input_string)?.1;
        assert_eq!(actual, 46);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let input_string = program();
        let sources = parse(&input_string, false)?;
        assert_eq!(sources.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            sources.mappings[Category::Soil as usize],
            Mapping {
                ranges: vec![MappingRange::new(98, 50, 2), MappingRange::new(50, 52, 48),]
            }
        );
        assert_eq!(
            sources.mappings[Category::Fertilizer as usize],
            Mapping {
                ranges: vec![
                    MappingRange::new(15, 0, 37),
                    MappingRange::new(52, 37, 2),
                    MappingRange::new(0, 39, 15),
                ]
            }
        );
        assert_eq!(
            sources.mappings[Category::Water as usize],
            Mapping {
                ranges: vec![
                    MappingRange::new(53, 49, 8),
                    MappingRange::new(11, 0, 42),
                    MappingRange::new(0, 42, 7),
                    MappingRange::new(7, 57, 4),
                ]
            }
        );
        assert_eq!(
            sources.mappings[Category::Light as usize],
            Mapping {
                ranges: vec![MappingRange::new(18, 88, 7), MappingRange::new(25, 18, 70),]
            }
        );
        assert_eq!(
            sources.mappings[Category::Temperature as usize],
            Mapping {
                ranges: vec![
                    MappingRange::new(77, 45, 23),
                    MappingRange::new(45, 81, 19),
                    MappingRange::new(64, 68, 13),
                ]
            }
        );
        assert_eq!(
            sources.mappings[Category::Humidity as usize],
            Mapping {
                ranges: vec![MappingRange::new(69, 0, 1), MappingRange::new(0, 1, 69),]
            }
        );
        assert_eq!(
            sources.mappings[Category::Location as usize],
            Mapping {
                ranges: vec![MappingRange::new(56, 60, 37), MappingRange::new(93, 56, 4),]
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_seed_ranges() -> Result<()> {
        let input_string = program();
        let sources = parse(&input_string, true)?;
        assert_eq!(
            sources.seeds,
            nofmt::pls! {
            vec![
                //First range
                79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92,
                //Second range
                55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67
            ]
            }
        );
        Ok(())
    }

    #[test]
    fn test_mapping() {
        let seed_to_soil = Mapping {
            ranges: vec![MappingRange::new(98, 50, 2), MappingRange::new(50, 52, 48)],
        };
        assert_eq!(seed_to_soil.map(0), 0);
        assert_eq!(seed_to_soil.map(50), 52);
        assert_eq!(seed_to_soil.map(51), 53);
        assert_eq!(seed_to_soil.map(96), 98);
        assert_eq!(seed_to_soil.map(97), 99);
        assert_eq!(seed_to_soil.map(98), 50);
        assert_eq!(seed_to_soil.map(99), 51);
    }

    #[test]
    fn test_all_mappings() -> Result<()> {
        let input_string = program();
        let sources = parse(&input_string, false)?;
        assert_eq!(sources.map_seed(79), [81, 81, 81, 74, 78, 78, 82]);
        assert_eq!(sources.map_seed(14), [14, 53, 49, 42, 42, 43, 43]);
        assert_eq!(sources.map_seed(55), [57, 57, 53, 46, 82, 82, 86]);
        assert_eq!(sources.map_seed(13), [13, 52, 41, 34, 34, 35, 35]);
        Ok(())
    }
}
