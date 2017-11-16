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

fn to_10bit(x:f32) -> i32 {
    let v=if x>=0.0 {
        (x/(10.0/512.0)) as i32 & 0x3FF
    }else{
        -((-x/(10.0/512.0)) as i32 & 0x3FF)
    };

    v
}

fn to_12bit(x:f32) -> i32 {
    let v=if x>=0.0 {
        (x/(10.0/2048.0)) as i32 & 0xFFF
    }else{
        -((-x/(10.0/2048.0)) as i32 & 0xFFF)
    };

    v
}

fn to_16bit(x:f32) -> i32 {
    let v=if x>=0.0 {
        (x/(10.0/32768.0)) as i32 & 0xFFFF
    }else{
        -((-x/(10.0/32768.0)) as i32 & 0xFFFF)
    };

    v
}

fn from10_to_16bit(x:i32) -> i32 {
    x*(1<<6)
}

fn from12_to_16bit(x:i32) -> i32 {
    x*(1<<4)
}

fn from_16bit_to_float(x:i32) -> f32 {
    let v=if x>=0 {
        (x as f32)*(10.0/32768.0)
    }else{
        (x as f32)*(10.0/32768.0)
    };

    v
}

fn cas16(x:i32) -> i32 {
    println!("{} {}",from_16bit_to_float(x),from_16bit_to_float(x));
    let v=from_16bit_to_float(x).cos()+from_16bit_to_float(x).sin();
    println!("{} {}",v,to_16bit(v));
    to_16bit(v)
}

pub fn dph(h:&[i32]) -> [i32;N] {//работает с 16 бит
    let mut H:[i32;N] = unsafe{std::mem::uninitialized()};

    for k in 0..N {
        let divN16=from12_to_16bit(to_12bit(1.0/N as f32));
        let pi16=from12_to_16bit(to_12bit(consts::PI));

        let b=h.iter().enumerate().fold(0, |H, (n,h)| {
            let a=cas16(2 * pi16 * (n as i32) * (k as i32));

            H + *h * a
        });

        println!("{} * {}",divN16,b);

        H[k]=divN16 * b
    }

    H
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process() {
        let signal=load_signal();
        let signal16:Vec<i32>=signal.iter().map(|x|
            from10_to_16bit(to_10bit(*x) )
        ).collect();

        let out_signal16:Vec<i32>=(N..LEN).map(|i| dph(&signal16[i-N..i])[0]).collect();

        let out_signal=out_signal16.iter().map(|x|
            from_16bit_to_float(*x)
        ).collect();

        write_signal(out_signal,"fixed.txt");
    }
}