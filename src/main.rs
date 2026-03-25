use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    sumtree::stub_info()?.generate()?;
    // The __init__ file is manually written, so remove that one.
    std::fs::remove_file("python/sumtree/__init__.pyi")?;
    println!("Generated Python stubs successfully.");
    Ok(())
}
