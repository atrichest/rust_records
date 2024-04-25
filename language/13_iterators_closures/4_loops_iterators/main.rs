fn main() {
    //let buffer: &mut [i32];
    let mut buffer: Vec<i32> = Vec::new();

    //let coefficients: [i64; 12];
    let mut coefficients: Vec<i64> = Vec::new();

    let qlp_shift: i16 = 0;

    for i in 1..37 {
        if i <= 12 {
            coefficients.push(i as i64);
        }
        buffer.push(i);
    }
    println!("{:?}", buffer);
    println!("{:?}", coefficients);

    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c + s as i64)
            .sum::<i64>()
            >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
        println!("{prediction} + {delta} = {}", buffer[i]);
    }
    println!("{:?}", buffer);

    let mut xs = [1, 2, 3];
    let ys = [4, 5, 6];

    println!("12 - 12..12 = (12 - 12)..12 = {:#?}", 12 - 12..12);
    println!("&xs[0..3]={:?}", &xs[0..3]);
    println!("&xs[1..3]={:?}", &xs[1..3]);
    println!("&xs[1..2]={:?}", &xs[1..2]);
    println!("&ys[0..3]={:?}", &ys[0..3]);

    let i = ys.iter().zip(&xs[1..2]);

    for a in i {
        println!("ys.iter().zip(&xs[1..2])-->{:?}", a);
    }

    let i = ys.iter().zip(&xs[1..3]);

    for a in i {
        println!("ys.iter().zip(&xs[1..3])-->{:?}", a);
    }
    let i = ys.iter().zip(&xs[0..3]);

    for a in i {
        println!("ys.iter().zip(&xs[0..3])-->{:?}", a);
    }

    let r: i16 = 0;
    for i in 1..xs.len() {
        let p = ys
            .iter()
            .zip(&xs[i - 1..i])
            .map(|(&c, &s)| c * s)
            .sum::<i32>();
        let delta = xs[i];
        xs[i] = p + delta;
        println!("{p} + {delta} = {}", xs[i]);
    }
    println!("{:?}", xs);
}
