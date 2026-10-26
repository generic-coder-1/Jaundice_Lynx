use std::{error::Error, f32::consts::PI, fs::File};

use clap::Parser;
use csv::Writer;
use impacted::{CollisionShape, Transform};
use rand::random;
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 0)]
    start_hares: u16,

    #[arg(long, default_value_t = 0)]
    start_lynx: u16,

    #[arg(long, default_value_t = 3)]
    min_hares: u16,

    #[arg(long, default_value_t = 1)]
    min_lynx: u16,

    #[arg(long, default_value_t = 120)]
    max_hares: u16,

    #[arg(long, default_value_t = 8)]
    max_lynx: u16,

    #[arg(long, default_value_t = 3)]
    hares_per_lynx: u16,

    #[arg(long, default_value_t = 1.0)]
    hares_size: f32,

    #[arg(long, default_value_t = 3.0)]
    lynx_size: f32,

    #[arg(short = 's', long, default_value_t = 12.0)]
    habitat_size: f32,

    #[arg(short, long, default_value_t = 25)]
    iterations: u16,

    #[arg(short, long, default_value_t = "output.csv".to_string())]
    output_file: String,
}

#[derive(Serialize)]
struct Record {
    num_lynx: u16,
    num_hares: u16,
    hares_captured: u16,
    hares_remaining: u16,
    lynx_starved: u16,
    lynx_survived: u16,
    lynx_offspring: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut wtr = Writer::from_path(&args.output_file)?;

    let mut hare_count = args.start_hares;
    let mut lynx_count = args.start_lynx;
    for _ in 0..args.iterations {
        hare_count = hare_count.clamp(args.min_hares, args.max_hares);
        lynx_count = lynx_count.clamp(args.min_lynx, args.max_lynx);
        
        let num_hares = hare_count;
        let num_lynx = lynx_count;

        let hares = generate_random_squares(num_hares as usize, args.hares_size, args.habitat_size);
        let lynx = generate_random_squares(num_lynx as usize, args.lynx_size, args.habitat_size);

        let hares_captured = count_touching(hares, lynx);
        let hares_remaining = num_hares - hares_captured;

        let lynx_survived = hares_captured.div_euclid(args.hares_per_lynx).min(num_lynx);
        let lynx_starved = num_lynx - lynx_survived;

        let hares_offspring = hares_remaining.min(args.max_hares - hares_remaining);
        let lynx_offspring = lynx_survived.min(args.max_lynx - lynx_survived);

        hare_count += hares_offspring;
        hare_count -= hares_captured;
        lynx_count += lynx_offspring;
        lynx_count -= lynx_starved;

        wtr.serialize(Record {
            num_lynx,
            num_hares,
            hares_captured,
            hares_remaining,
            lynx_starved,
            lynx_survived,
            lynx_offspring,
        })?;
    }

    wtr.flush()?;
    drop(wtr);
    print_csv(args.output_file)?;

    Ok(())
}

fn count_touching(targets: Vec<CollisionShape>, detectors: Vec<CollisionShape>) -> u16 {
    let mut count = 0;
    'target: for target in targets {
        for detector in &detectors {
            if target.is_collided_with(detector) {
                count += 1;
                continue 'target;
            }
        }
    }
    count
}

fn generate_random_squares(count: usize, square_size: f32, area_size: f32) -> Vec<CollisionShape> {
    (0..count)
        .map(|_| {
            CollisionShape::new_rectangle(square_size, square_size).with_transform(
                Transform::from_angle_translation(
                    2.0 * PI * random::<f32>(),
                    [area_size * random::<f32>(), area_size * random::<f32>()],
                ),
            )
        })
        .collect()
}

fn print_csv(file_path: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    if let Ok(headers) = rdr.headers() {
        println!("Headers: {:?}", headers);
    }

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", &record);
    }

    Ok(())
}
