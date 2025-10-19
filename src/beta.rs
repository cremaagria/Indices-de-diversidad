#[allow(dead_code)]
pub fn matriz_binaria(i: &[f64]) -> Vec<bool> {
    i.iter().map(|x| *x > 0.0).collect()
}

#[allow(dead_code)]
pub fn similitud(data: &Vec<Vec<f64>>) {
    println!("jaccard_cualitativo");
    for row in 0..data.len() {
        let mut fila: Vec<f64> = Vec::new();
        for row1 in 0..data.len() {
            fila.push(round_n(jaccard_cualitativo(&data[row], &data[row1]), 5));
        }
        println!("\t{:?}", fila);
    }

    println!("sorensen_cualitativo");
    for row in 0..data.len() {
        let mut fila: Vec<f64> = Vec::new();
        for row1 in 0..data.len() {
            fila.push(round_n(sorensen_cualitativo(&data[row], &data[row1]), 5));
        }
        println!("\t{:?}", fila);
    }

    println!("sorensen_cuantitativo");
    for row in 0..data.len() {
        let mut fila: Vec<f64> = Vec::new();
        for row1 in 0..data.len() {
            fila.push(round_n(sorensen_cuantitativo(&data[row], &data[row1]), 5));
        }
        println!("\t{:?}", fila);
    }

    println!("morisita_orn");
    for row in 0..data.len() {
        let mut fila: Vec<f64> = Vec::new();
        for row1 in 0..data.len() {
            fila.push(round_n(morisita_orn(&data[row], &data[row1]), 5));
        }
        println!("\t{:?}", fila);
    }
}

// Jaccard cualitativo (qualitative)
#[allow(dead_code)]
pub fn jaccard_cualitativo(i: &[f64], j: &[f64]) -> f64 {
    let i = matriz_binaria(i);
    let j = matriz_binaria(j);
    let mut intersection = 0usize;
    let mut union = 0usize;

    for (a, b) in i.iter().zip(j.iter()) {
        if *a && *b {
            intersection += 1;
        }
        if *a || *b {
            union += 1;
        }
    }

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

// Sørensen cuantitativo (quantitative)
#[allow(dead_code)]
pub fn sorensen_cuantitativo(i: &[f64], j: &[f64]) -> f64 {
    let sum_min: f64 = i.iter().zip(j.iter()).map(|(a, b)| a.min(*b)).sum();
    let sum_total: f64 = i.iter().sum::<f64>() + j.iter().sum::<f64>();

    if sum_total == 0.0 {
        0.0
    } else {
        (2.0 * sum_min) / sum_total
    }
}

// Sørensen cualitativo (qualitative)
#[allow(dead_code)]
pub fn sorensen_cualitativo(i: &[f64], j: &[f64]) -> f64 {
    let i = matriz_binaria(i);
    let j = matriz_binaria(j);
    let mut interseccion = 0;
    let mut a1 = 0;
    let mut b2 = 0;

    for (a, b) in i.iter().zip(j.iter()) {
        if *a {
            a1 += 1;
        }
        if *b {
            b2 += 1;
        }
        if *a && *b {
            interseccion += 1;
        }
    }

    if a1 + b2 == 0 {
        0.0
    } else {
        2.0 * (interseccion as f64 / (a1 as f64 + b2 as f64))
    }
}
fn round_n(x: f64, n: u32) -> f64 {
    let factor = 10f64.powi(n as i32);
    (x * factor).round() / factor
}

#[allow(dead_code)]
pub fn morisita_orn(i: &[f64], j: &[f64]) -> f64 {
    let an: f64 = i.iter().sum();
    let bn: f64 = j.iter().sum();
    let ani_bni: f64 = i.iter().zip(j).map(|(a, b)| a * b).sum();
    let ani2: f64 = i.iter().map(|a| a * a).sum();
    let bni2: f64 = j.iter().map(|b| b * b).sum();
    let da = ani2 / (an * an);
    let db = bni2 / (bn * bn);
    2.0 * ani_bni / ((da + db) * (an * bn))
}
