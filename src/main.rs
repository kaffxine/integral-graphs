pub mod graph;
pub use graph::AdjMatrix;

pub mod nauty;
pub use nauty::labelg;

pub mod spectral;
pub use spectral::is_integral;

pub mod matrix;
pub use matrix::Matrix;

pub mod database;
pub use database::Database;

use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::mem::size_of::<usize>() != 8 {
        panic!("do not run this program on non-64-bit arch");
    }

    test_generate();


    Ok(())
}

fn keep_generating(n_nodes: u64, max_degree: u32, db: Arc<Mutex<Database>>) -> Result<(), String> {

    Ok(())
}

fn generate(n_nodes: u64, max_degree: u32, seed: usize) -> Result<AdjMatrix, String> {
    let big_prime_1: usize = 18446744073709551557;
    let big_prime_2: usize = 18446744073709551533;
    let mut noise = seed;

    let mut adjm = AdjMatrix::complete(n_nodes)?;
    let n = n_nodes as usize;
    let max_degree = max_degree as usize;

    let mut neighbors_left = vec![n - 1; n];

    let mut ready: usize = 0;
    let mut active: usize = n - 1;
    while ready < n {
        active += 1;
        if active == n {
            active = 0;
        }

        if neighbors_left[active] <= max_degree {
            continue;
        }

        noise = noise * big_prime_1 + big_prime_2;
        let other = noise % n;

        if active == other || neighbors_left[other] == 1 {
            continue;
        }

        if adjm.is_edge(active as u32, other as u32)? {
            adjm.set(active as u32, other as u32, false)?;
            neighbors_left[active] -= 1;
            neighbors_left[other] -= 1;
            if neighbors_left[active] == max_degree {
                ready += 1;
            }
            if neighbors_left[other] == max_degree {
                ready += 1;
            }
        }

    }
    return Ok(adjm);
}

fn test_generate() -> Result<(), String> {
    for i in 0..20 {
        let adjm = generate(13, 4, 69420 + i)?;
        let mt: Matrix = adjm.try_into()?;
        println!("{mt}");
    }
    Ok(())
}


fn test_integral_detection_in_circular_graphs() -> Result<(), String> {
    for n in 2..15 {
        let mut c = AdjMatrix::empty(n as u64)?;
        for i in 0..n - 1 {
            c.set(i, i + 1, true)?;
        }
        c.set(0, n - 1, true)?;
        println!("{c}");
        if is_integral(c)? {
            println!("INTEGRAL!");
        } else {
            println!("boring");
        }
    }
    Ok(())
}

fn test_nauty_integration_and_proper_labeling() -> Result<(), String> {
    for i in (1..62) {
        let adjm = AdjMatrix::random(i, 0.5)?;
        let mut found = false;
        let mut all_iso = true;
        let labeled = labelg(adjm.to_graph6()?)?;
        for am in adjm.permutations().into_iter() {
            let g6 = am.to_graph6()?;
            if labeled == g6 {
                found = true;
            }
            if labelg(g6)? != labeled {
                all_iso = false;
            }
        }
        print!("{i}-node: ");
        if found && all_iso {
            println!("all as expected");
        }
        else {
            println!("wrong: found={found} all_iso={all_iso}");
        }
    }
    Ok(())
}
