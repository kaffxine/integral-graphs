pub mod graph;
pub use graph::AdjMatrix;

pub mod nauty;
pub use nauty::labelg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::mem::size_of::<usize>() != 8 {
        panic!("do not run this program on non-64-bit arch");
    }
    
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
