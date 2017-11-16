use std;
use rand;

use std::f32::consts;
use rand::distributions::{IndependentSample, Range};
/*
15. Реализовать ДПХ при N = 8 – 1024
n1 =10, n2 =12, n3 = 16
*/

use super::N;
use super::LEN;
use signal::{load_signal,write_signal};

fn cas(x:f32) -> f32 {
    x.cos()+x.sin()
}

pub fn dph(h:&[f32]) -> [f32;N] {
    let mut H:[f32;N] = unsafe{std::mem::uninitialized()};

    for k in 0..N {
        H[k]=1.0/N as f32 * h.iter().enumerate().fold(0.0, |H, (n,h)| {
            let a=cas(2.0 * consts::PI/(N as f32) * (n as f32) * (k as f32));

            H + *h * a
        })
    }

    H
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process() {
        let signal=load_signal();

        let out_signal=(N..LEN).map(|i| dph(&signal[i-N..i])[0]).collect();

        write_signal(out_signal,"float.txt");
    }
}