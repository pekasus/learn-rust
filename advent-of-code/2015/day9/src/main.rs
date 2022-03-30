use itertools::{Itertools, Permutations};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;
use std::str::Lines;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    // println!("{:#?}", get_city_data(lines))
    let city_data = get_city_data(lines);
    let cities = get_cities_set(&city_data);

    // 0..cities.len()
    let city_index_map =
        cities
            .iter()
            .fold(HashMap::<&String, usize>::new(), |mut result, city| {
                result.insert(city, result.len());
                result
            });

    let city_index_vec = cities
        .iter()
        .map(|&city_name| city_name.to_owned())
        .collect::<Vec<String>>();

    // println!("cities: {:#?}", cities);
    let full_routes = add_reverse_distances(&city_data);
    // println!("{:#?}", full_routes);

    let all_distances = add_reverse_distances(&city_data);

    let city_permutations = (0u8..cities.len() as u8).permutations(cities.len());

    println!(
        "Shortest: {}",
        get_total_distance(
            city_permutations.clone(),
            &city_index_vec,
            &full_routes,
            true
        )
    );

    println!(
        "Longest: {}",
        get_total_distance(
            city_permutations.clone(),
            &city_index_vec,
            &full_routes,
            false
        )
    )
}

fn get_total_distance(
    city_permutations: Permutations<Range<u8>>,
    city_index_vec: &Vec<String>,
    full_routes: &HashMap<(&String, &String), &u16>,
    is_shortest: bool,
) -> u16 {
    let mut best = if is_shortest { u16::MAX } else { u16::MIN };

    city_permutations.for_each(|order| {
        let total_distance_result = order.windows(2).try_fold(0u16, |mut acc, window| {
            let from_idx = window[0];
            let to_idx = window[1];
            let key = &(
                &city_index_vec[from_idx as usize],
                &city_index_vec[to_idx as usize],
            );
            let distance = **full_routes.get(key).unwrap();
            acc += distance;
            if is_shortest && acc > best {
                Err(())
            } else {
                Ok(acc)
            }
        });

        if total_distance_result.is_ok() {
            let total_distance = total_distance_result.unwrap();
            if is_shortest && total_distance < best {
                best = total_distance;
                // dbg!(best);
            } else if !is_shortest && total_distance > best {
                best = total_distance;
                // dbg!(best);
            }
        }
    });
    best
}

///                       -> city_idx ->                 -> all_distances lookup by (String, String)
/// (from:usize, to: usize) ->          (from_name, to_name)                    -> distance: u16
///
/// or, if we map string names to indexes initially, including all_distances:
///                            -> all distancies looked up by (usize, usize)
///                           /                                       \
/// (from:usize, to:usize)  ->                                          > distance: u16

fn get_city_data(lines: Lines) -> Vec<(String, String, u16)> {
    lines
        .map(|line| {
            let line: Vec<&str> = line.split_whitespace().collect();
            (
                line[0].to_owned(),
                line[2].to_owned(),
                line[4].parse().unwrap(),
            )
        })
        .collect()
}

fn get_cities_set<'a>(city_data: &'a Vec<(String, String, u16)>) -> HashSet<&'a String> {
    let result = city_data.iter().fold(
        HashSet::<&'a String>::new(),
        move |mut set, (from, to, _)| {
            set.insert(from);
            set.insert(to);
            set
        },
    );
    result
}

fn add_reverse_distances<'a>(
    city_data: &'a Vec<(String, String, u16)>,
) -> HashMap<(&'a String, &'a String), &'a u16> {
    let output: HashMap<(&String, &String), &u16> = HashMap::new();
    city_data.iter().fold(output, |mut acc, (from, to, dist)| {
        acc.insert((from, to), dist);
        acc.insert((to, from), dist);
        acc
    })
}

/*

1. count different cities
2. make set, with each city being index. ex. London = 0
3. make list of lists of every possible order combination of indices
4. calculate each path in order

{London, Dublin, Belfast} = [0, 1, 2]

[ [0, 1, 2], [0, 2, 1], [1, 2, 0], [1, 0, 2], [2, 0, 1], [2, 1, 0] ]

clean list by removing "duplicates" with the same order in reverse

        London Dublin Belfast
London      0    350    275
Dublin     350    0     100
Belfast    275   100     0

[0, 1, 2] -> [350, 100] -> sum(distances)

*/
