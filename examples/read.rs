use clap::Parser;
use ndarray::{ArrayView2, s};
use ndarray_npy::write_npy;
use sidd_rs::read_sidd;
use std::path::Path;

/// Example of reading and working with a SIDD file
#[derive(Parser)]
struct Args {
    /// Input file
    input: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let sidd = read_sidd(&args.input).unwrap();
    let meta = sidd.meta.get_v1_0_0_meta().unwrap();
    println!("{:#?}", meta.exploitation_features);
    std::process::exit(0);

    let arr = sidd.image_data[0].array.slice(s![0..3, 0..3]);
    arr.indexed_iter()
        .for_each(|((row, col), val)| println!("[{row}, {col}] = {}", val));

    let img = &sidd.image_data[0].array;
    dbg!(&img.raw_dim());

    if let Err(e) = save_as_npy(img.view(), &args.output) {
        eprintln!("Error saving npy: {}", e)
    }
}

pub fn save_as_npy<P: AsRef<Path>>(
    array: ArrayView2<u8>,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert ArrayView2 to ArrayD (owned array)
    let owned_array = array.to_owned();

    // Save as NPY
    write_npy(path, &owned_array)?;

    Ok(())
}
