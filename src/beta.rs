#[allow(dead_code)]
pub fn matriz_binaria(i: &[i32]) -> Vec<bool> {
    i.iter().map(|x| *x > 0).collect()
}

#[allow(dead_code)]
pub fn jaccard(i: &[i32], j: &[i32]) -> f64 {
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
