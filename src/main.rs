pub mod graph;
pub use graph::AdjMatrix;

pub mod nauty;
pub use nauty::labelg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::mem::size_of::<usize>() != 8 {
        panic!("do not run this program on non-64-bit arch");
    }
    
    let n = 5;
    let adjm = AdjMatrix::random(n, 0.5)?;
    let perms = adjm.permutations();
    for p in perms {
        let g = p.graph6()?;
        let c = labelg(g.clone())?;
        println!("{} {}", g, c);
    }

    Ok(())
}
