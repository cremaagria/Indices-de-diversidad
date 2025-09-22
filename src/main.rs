use std::ops::Neg;

fn main() {
    //let especies: Vec<&str> = Vec::from(["osos", "panteras", "jaguares", "antilopes"]);
    //
    let avistamientos: Vec<f64> = [10, 10, 10, 10, 10, 10].iter().map(|&x| x as f64).collect();

    println!("la riqueza de especies es {}", avistamientos.len());
    let shannon = alphasimpson_1(&avistamientos);
    println!("la diversidad de shannon es : {:?}", shannon);
}

//  (0-[1- {1/S} ]) entre mas cercano a 0 mayor diversidad de especies
//
fn alphasimpson(avistamientos: &Vec<f64>) -> f64 {
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
fn alphasimpson_1(avistamientos: &Vec<f64>) -> f64 {
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
fn alphasimpson_inverso(avistamientos: &Vec<f64>) -> f64 {
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
fn alphashannon(avistamientos: &Vec<f64>) -> f64 {
    println!(
        "valor maximo posible de shannon es :{}",
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

