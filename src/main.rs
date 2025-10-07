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

// jaccard ----
fn matrizBinaria(i: &[i32]) -> Vec<bool> {
    i.iter().map(|x| *x > 0).collect()
}

fn jaccard(i: &[i32], j: &[i32]) {
    let i2 = matrizBinaria(i);
    let j2 = matrizBinaria(j);
    
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

    println!("{}", jaccard_index);
}


fn llamado() {
    // mejorar para que reciba un csv y transformar para que me haga un reporte
    let  v: Vec<i32> = vec![1, 0, 0, 3, 1];
    let j: Vec<i32> = vec![1, 0, 0, 3, 0];
    let l: Vec<i32> = vec![0, 1, 4, 4, 1];
    
    
    // imprimirlo en forma de heap map
    jaccard(&v, &l);
    jaccard(&j, &l)
}

///----
