use std::ops::Neg;

pub fn alphasimpson(avistamientos: &Vec<f64>) -> f64 {
    let sum: f64 = avistamientos.iter().sum();
    avistamientos
        .iter()
        .map(|e| {
            let p = e / sum;
            p.powf(2.0)
        })
        .sum::<f64>()
}

// valores entre cercanos al uno son mas diversos mas cercanos al cero menos diversos
pub fn alphasimpson_1(avistamientos: &Vec<f64>) -> f64 {
    let sum: f64 = avistamientos.iter().sum();

    let d: f64 = avistamientos
        .iter()
        .map(|e| {
            let p = e / sum;
            p.powf(2.0)
        })
        .sum::<f64>();
    1.0 - d
}
// valores entre 1 y s
pub fn alphasimpson_inverso(avistamientos: &Vec<f64>) -> f64 {
    let sum: f64 = avistamientos.iter().sum();
    let d: f64 = avistamientos
        .iter()
        .map(|e| {
            let p = e / sum;
            p.powf(2.0)
        })
        .sum::<f64>();
    1.0 / d
}

// (0-lns)
pub fn alphashannon(avistamientos: &Vec<f64>) -> f64 {
    println!(
        "valor maximo posible de shannon deberia ser :{}",
        (avistamientos.len() as f64).ln()
    );
    let sum: f64 = avistamientos.iter().sum();
    avistamientos
        .iter()
        .map(|e| {
            let p = e / sum;
            if p > 0.0 { p * p.ln() } else { 0.0 }
        })
        .sum::<f64>()
        .neg()
}
