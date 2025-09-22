use std::ops::Neg;

fn main() {
    //let especies: Vec<&str> = Vec::from(["osos", "panteras", "jaguares", "antilopes"]);
    //
    let avistamientos: Vec<f64> = [1, 1].iter().map(|&x| x as f64).collect();

    println!("la riqueza de especies es {}", avistamientos.len());
    let shannon = alphashannon(&avistamientos);
    println!("la diversidad de shannon es : {:?}", shannon);
}

/*  (0-[1-1/S])
fn alphasimpson(especies: &Vec<&str>, avistamientos: &[u32]) {
    true
}
*/

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
