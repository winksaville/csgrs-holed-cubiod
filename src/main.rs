use clap::{CommandFactory, Parser, value_parser};
use csgrs::{float_types::Real, traits::CSG};
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
    #[arg(short, long, default_value = "20.0")]
    length: Real,

    /// height of the panel
    #[arg(short = 'H', long, default_value = "5.0")]
    height: Real,

    /// hole diameter, zero for no hole
    #[arg(short, long, default_value = "0")]
    diameter: Real,

    /// number of segments to use when creating the tube, minimum is 3
    #[arg(short, long, default_value = "50", value_parser = value_parser!(u32).range(3..), help = "The number of segments to use when creating the tube, minimum is 3")]
    segments: u32,
}

fn main() {
    let args = Args::parse();
    //println!("args: {:?}", args);

    let clap_metadata = Args::command();
    println!(
        " arg.version: {}",
        clap_metadata.get_version().unwrap_or_default()
    );

    // Simple code
    let cuboid_dimensions = Vector3::new(args.width, args.length, args.height);
    println!("outside_dimenstions: {:?}", cuboid_dimensions);
    let mut cuboid = Mesh::cuboid(
        cuboid_dimensions.x,
        cuboid_dimensions.y,
        cuboid_dimensions.z,
        None,
    );

    // Someday union a hole support cylinder with the cubiod if requested
    // this doesn't work because I can't get prusa-clicer to do 100% infill
    // for the hole support cylinder. The workaround is to probably use
    // a modifier cylinder in prusa-slicer and indicating 100% infill

    let mut hole_diameter_file_name = String::new();
    // Create a hole in the cuboid if diameter is greater than zero
    if args.diameter > 0.0 {
        let hole_diameter = args.diameter;
        let hole_radius = hole_diameter / 2.0;

        let center_location = Vector3::new(
            cuboid_dimensions.x / 2.0,
            cuboid_dimensions.y / 2.0,
            cuboid_dimensions.z / 2.0,
        );

        let hole = Mesh::cylinder(
            hole_radius,
            args.height,
            args.segments as usize,
            None,
        ).translate(center_location.x, center_location.y, 0.0);

        cuboid = cuboid.difference(&hole);

        hole_diameter_file_name = format!("_d-{:.2}", args.diameter);
    }

    // Write the result as an ASCII STL:
    let name = "holed-cuboid".to_owned()
        + &format!("_w-{:.2}", args.width)
        + &format!("_l-{:.2}", args.length)
        + &format!("_h-{:.2}", args.height)
        + &hole_diameter_file_name;
    let stl = cuboid.to_stl_ascii(&name);
    let file_name = name + ".stl";
    println!("Writing STL file: {}", file_name);
    std::fs::write(file_name, stl).unwrap();
}
