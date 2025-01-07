pub mod graph;
pub use graph::AdjMatrix;

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
    println!("{}", adjm.graph6()?);

    Ok(())
}
