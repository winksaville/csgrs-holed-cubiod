use clap::{CommandFactory, Parser};
use csgrs::float_types::Real;
use nalgebra::Vector3;

type Mesh = csgrs::mesh::Mesh<()>;

/// Generate a cubiod mesh with holes and write it to an STL file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// width of the panel
    #[arg(short, long, default_value = "10.0")]
    width: Real,

    /// length of the panel
    #[arg(short, long, default_value = "100.0")]
    length: Real,

    /// height of the panel
    #[arg(short = 'H', long, default_value = "5.0")]
    height: Real,

    /// hole diameter, zero for no hole
    #[arg(short = 'd', long, default_value = "0")]
    diameter: Real,

    /// hole support thickness
    #[arg(short = 's', long, default_value = "0")]
    support_thickness: Real,
}

fn main() {
    let args = Args::parse();
    //println!("args: {:?}", args);

    let clap_metadata = Args::command();
    println!(
        " arg.version: {}",
        clap_metadata.get_version().unwrap_or_default()
    );

    let outside: Mesh;

    // Simple code
    let cuboid_dimensions = Vector3::new(args.width, args.length, args.height);
    println!("outside_dimenstions: {:?}", cuboid_dimensions);
    outside = Mesh::cuboid(
        cuboid_dimensions.x,
        cuboid_dimensions.y,
        cuboid_dimensions.z,
        None,
    );


    // Write the result as an ASCII STL:
    let name = "holed-cuboid".to_owned()
        + &format!("_w-{:.2}", args.width)
        + &format!("_l-{:.2}", args.length)
        + &format!("_h-{:.2}", args.height);
    let stl = outside.to_stl_ascii(&name);
    let file_name = name + ".stl";
    println!("Writing STL file: {}", file_name);
    std::fs::write(file_name, stl).unwrap();
}
