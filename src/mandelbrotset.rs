use num::Complex;

pub fn square_loop(mut x:f64){
    loop{
        x = x * x;
    }
}

pub fn sqaure_add_loop(c: f64){
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

pub fn  complex_square_add_loop(c:Complex<f64>){
    let mut z =  Complex{re:0.0, im:0.0};
    loop {
        z = z * z + c;
    }
    
}

pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit{
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    Non
}