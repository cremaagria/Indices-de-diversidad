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

// Indices Beta

// Indice Jaccard

#[allow(dead_code)]
fn matriz_binaria(i: &[i32]) -> Vec<bool> {
    i.iter().map(|x| *x > 0).collect()
}

#[allow(dead_code)]
fn jaccard(i: &[i32], j: &[i32]) -> f64 {
    let i2 = matriz_binaria(i);
    let j2 = matriz_binaria(j);

    let mut intersection = 0usize;
    let mut union = 0usize;

    for (a, b) in i2.iter().zip(j2.iter()) {
        if *a || *b {
            union += 1;
            if *a && *b {
                intersection += 1;
            }
        }
    }

    let jaccard_index = if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    };

    //println!("{}", jaccard_index);
    jaccard_index
}
