use pestle::gen::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for a in args {
        let p = pestle::parse(&a)?;
        let b = bumpalo::Bump::new();
        let t = Sequence::build(p, &b);
        println!("{t:#?}");
    }
    Ok(())
}
