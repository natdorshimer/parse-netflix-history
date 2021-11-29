use std::io::BufReader;
use serde::Deserialize;
use std::fs::File;
use std::collections::HashMap;
use itertools::Itertools;
use std::env;

fn main() {
  let csv_file_name = match env::args().collect::<Vec<String>>().get(1) {
    Some(file_name) => file_name.to_owned(),
    None => "netflixhistory.csv".to_string()
  };

  match File::open(&csv_file_name) {
    Ok(csv_file) => parse_netflix_titles(csv_file),
    Err(_) => println!("Could not find file with name {}", &csv_file_name)
  };
}

#[derive(Deserialize)]
struct Record {
  title: String,
}

fn parse_netflix_titles(csv_file: File) {
  let reader = BufReader::new(csv_file);
  let mut csv_reader = csv::Reader::from_reader(reader);
  let endings = ["Limited", "Season", ": Book", "Part", ": "];  

  let filtered_titles = csv_reader
    .deserialize()
    .filter_map(|row: Result<Record, csv::Error>| row.ok())
    .map(|row| row.title)
    .map(|title| cut_off(&title, &endings))
    .collect::<Vec<String>>();

  to_frequency_map(&filtered_titles)
    .into_iter()
    .sorted_by(|(_, freq1), (_, freq2)| freq2.cmp(freq1))
    .for_each(|(title, freq)| println!("{}, {}", title, freq));
}

fn to_frequency_map<T>(filtered_titles: &[T]) -> HashMap<T, u32> where 
    T: std::hash::Hash,
    T: std::cmp::Eq,
    T: Clone {
  filtered_titles
    .iter()
    .fold(HashMap::new(), |mut map, key| {
      *map.entry(key.clone()).or_insert(0) += 1;
      map
    })
}

fn cut_off(input: &str, endings: &[&str]) -> String {
  endings
    .iter()
    .fold(input.to_string(), |acc, ending| {
        acc
          .split_terminator(ending)
          .next()
          .unwrap()
          .to_string()
  })
}
