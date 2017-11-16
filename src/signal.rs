use std;
use rand;

use rand::distributions::{IndependentSample, Range};

use std::io::{BufReader,BufRead};
use std::io::{BufWriter,Write};
use std::fs::File;

use super::N;
use super::LEN;

#[derive(Clone)]
enum State {
    Strob{until:usize},
    Zero
}

fn rand(i:usize) -> f32 {
    60.0 + i as f32 *7.3 % 40.0
}

pub fn gen_signal(){
    let between = Range::new(-10.0, 10.0);
    let mut rng = rand::thread_rng();

    let mut state=State::Zero;
    let mut period=0;

    let signal=(0..LEN).map(
        |i| {
            match state {
                State::Zero => {
                    if i>=period {
                        state=State::Strob{until:i+i/4};
                        period=i*2;
                    }

                    0.0
                },
                State::Strob {until} => {
                    if i>=until {
                        state=State::Zero;
                    }

                    //rand(i)
                    between.ind_sample(&mut rng)
                }
            }
        }
    ).collect();

    write_signal(signal,"signal.txt");
}

pub fn load_signal() -> Vec<f32>{
    let f = File::open("signal.txt").unwrap();
    let mut reader = BufReader::new(f);

    let signal:Vec<f32>=reader.lines().map( |line| {
        line.unwrap().parse::<f32>().unwrap()
    }).collect();

    assert_eq!(signal.len(),LEN);

    signal
}

pub fn write_signal(signal:Vec<f32>,filename:&str) {
    let f = File::create(filename).unwrap();
    let mut writer = BufWriter::new(f);

    for x in signal.iter() {
        let line=format!("{}\n",x);
        writer.write_all(line.as_bytes()).unwrap();
    }
}