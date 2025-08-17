use clap::{CommandFactory, Parser, ValueEnum, value_parser};
use csgrs::{float_types::Real, traits::CSG};
use nalgebra::Vector3;

type Mesh = csgrs::mesh::Mesh<()>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Amf,
    Dxf,
    Obj,
    Ply,
    StlAscii,
    StlBinary,
}

/// Generate a cubiod mesh with holes and write it to an STL file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// length of the cuboid
    #[arg(short, long, default_value = "20.0")]
    length: Real,

    /// width of the cuboid
    #[arg(short, long, default_value = "10.0")]
    width: Real,

    /// height of the cuboid
    #[arg(short = 'H', long, default_value = "5.0")]
    height: Real,

    /// hole diameter, zero for no hole
    #[arg(short, long, default_value = "0")]
    diameter: Real,

    /// number of segments to use when creating the tube, minimum is 3
    #[arg(
        short,
        long, default_value = "3",
        value_parser = value_parser!(u32).range(3..),
        help = "The number of segments to use when creating the tube, minimum is 3"
    )]
    segments: u32,

    /// Output format for the mesh file
    #[arg(
        short,
        long,
        default_value = "obj",
        value_enum,
        help = "Output format for the mesh file"
    )]
    output_format: OutputFormat,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

        let hole = Mesh::cylinder(hole_radius, args.height, args.segments as usize, None)
            .translate(center_location.x, center_location.y, 0.0);

        cuboid = cuboid.difference(&hole);

        hole_diameter_file_name = format!("_s-{}_d-{:.2}", args.segments, args.diameter);
    }

    // Write the result as an ASCII STL:
    let name = "holed-cuboid".to_owned()
        + &format!("_l-{:.2}", args.length)
        + &format!("_w-{:.2}", args.width)
        + &format!("_h-{:.2}", args.height)
        + &hole_diameter_file_name;

    let shape: Vec<u8>;
    let file_name: String;
    (shape, file_name) = match args.output_format {
        OutputFormat::Amf => {
            let s = cuboid.to_amf(&name, "millimeter").into();
            let f = name + ".amf";
            (s, f)
        }
        OutputFormat::Obj => {
            let s = cuboid.to_obj(&name).into();
            let f = name + ".obj";
            (s, f)
        }
        OutputFormat::Dxf => {
            let s = cuboid.to_dxf()?;
            let f = name + ".dxf";
            (s, f)
        }
        OutputFormat::Ply => {
            let s = cuboid.to_ply(&name).into();
            let f = name + ".ply";
            (s, f)
        }
        OutputFormat::StlAscii => {
            let s = cuboid.to_stl_ascii(&name).into();
            let f = name + ".stl";
            (s, f)
        }
        OutputFormat::StlBinary => {
            let s = cuboid.to_stl_binary(&name)?;
            let f = name + ".stl";
            (s, f)
        }
    };
    println!("Writing file: {}", file_name);
    std::fs::write(&file_name, shape).unwrap();

    Ok(())
}
