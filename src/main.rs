use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    i64,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Station {
    min: i64,
    sum: i64,
    max: i64,
    count: i64,
}

impl Station {
    fn add(&mut self, value: i64) {
        if value > self.max {
            self.max = value
        }
        if value < self.min {
            self.min = value
        }
        self.sum += value;
        self.count += 1;
    }

    fn avg(&self) -> i64 {
        return self.sum / self.count;
    }

    fn default() -> Self {
        Self {
            min: i64::MAX,
            sum: 0,
            max: i64::MIN,
            count: 0,
        }
    }

    fn new(value: i64) -> Self {
        let mut station = Self::default();
        station.add(value);
        station
    }
}

impl Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/", format_float(self.min));
        write!(f, "{}/", format_float(self.avg()));
        write!(f, "{}", format_float(self.max))
    }
}

fn parse_int(buf: &str) -> i64 {
    let mut number: i64 = 0;
    for c in buf.chars() {
        if c == '-' {
            number *= -1;
        } else if c >= '0' && c <= '9' {
            number *= 10;
            number += c as i64 - 48;
        }
    }
    number
}

fn format_float(value: i64) -> String {
    let mut s = (value / 1000).to_string();
    match s.len() {
        1 => s,
        2 => format!("0.{}", s),
        l => {
            s.insert(l - 1, '.');
            s
        },
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    match args.len() {
        0 | 1 => panic!("Missing <csv path>"),
        _ => {}
    }

    let file_name = &args[1];
    let mut file = File::open(file_name).expect("File not found");

    naive(&mut file);
}

fn naive(file: &mut File) {
    let mut reader = BufReader::new(file);

    let mut stations = HashMap::<String, Station>::new();

    let mut buf = String::with_capacity(128);
    let mut line = 0u64;

    while let Ok(size) = reader.read_line(&mut buf) {
        if size == 0 {
            break;
        }

        let lenght = buf.find(";");
        if lenght.is_none() {
            continue;
        }
        let lenght = lenght.unwrap();

        if let Some(station) = stations.get_mut(&buf[..lenght]) {
            station.add(parse_int(&buf[(lenght + 1)..]));
            // println!("{} : {:.?}", &buf[..lenght], &station);
        } else {
            let station = Station::new(parse_int(&buf[(lenght + 1)..]));
            // println!("{} : {:.?}", &buf[..lenght], &station);
            stations.insert(buf[..lenght].into(), station);
        }

        line += 1;
        buf.clear();
    }

    print!("{{");
    for (name, station) in stations.iter() {
        print!("{}={}, ", name, station.to_string());
    }
    print!("}}");
}
