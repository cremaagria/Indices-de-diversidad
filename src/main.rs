use std::{fs, io::Read};
mod alpha;
mod beta;

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

    println!("\n    ╔═══════════════════════════════════════════════════════╗");
    println!("    ║             INDICES DE DIVERSIDAD ALPHA               ║");
    println!("    ╚═══════════════════════════════════════════════════════╝\n");

    for row in &matrix {
        println!("Datos: {:?}", row);

        println!("\tDiversidad shannon: {}", alpha::alphashannon(row));
        println!("\tDiversidad alphasimpson: {}", alpha::alphasimpson(row));
        println!(
            "\tDiversidad alphasimpson-1: {}",
            alpha::alphasimpson_1(row)
        );
        println!(
            "\tDiversidad alphasimpson_inverso: {}",
            alpha::alphasimpson_inverso(row)
        );

        println!("------------------------------------------------------------------------");
    }
}
