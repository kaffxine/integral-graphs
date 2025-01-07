pub mod graph;
pub use graph::AdjMatrix;

pub mod nauty;
pub use nauty::labelg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::mem::size_of::<usize>() != 8 {
        panic!("do not run this program on non-64-bit arch");
    }
    
    let n = 7;
    let adjm = AdjMatrix::random(n, 0.5)?;
    println!("{:?}", adjm);
    println!("{}", adjm);
    println!("{:?}", adjm.adj_lists());
    println!("{}", adjm.base64());
    let g6 = adjm.graph6()?;
    let cano = labelg(g6.clone())?;
    println!("g6={} cano={}", g6, cano);

    Ok(())
}
