use std::ops::Neg;
use std::{fs, io::Read};

fn main() {
    // Leer archivo completo
    let mut file = fs::File::open("Prueba3.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut matrix: Vec<Vec<f64>> = Vec::new();

    for line in s.lines().skip(1) {
        // omite encabezado
        let nums: Vec<f64> = line
            .split(',')
            .filter_map(|x| x.trim().parse().ok())
            .collect();

        if !nums.is_empty() {
            matrix.push(nums);
        }
    }

    println!("Matriz cuadrada:");
    for row in &matrix {
        println!("Datos: {:?}", row);

        println!("Diversidad shannon: {}", alphashannon(row));
        println!("Diversidad alphasimpson: {}", alphasimpson(row));
        println!("Diversidad alphasimpson-1: {}", alphasimpson_1(row));
        println!(
            "Diversidad alphasimpson_inverso: {}",
            alphasimpson_inverso(row)
        );
        println!("---------------------------");
    }
}

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
