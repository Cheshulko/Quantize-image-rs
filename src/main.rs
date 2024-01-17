use std::{
    collections::HashSet,
    env,
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
    time::Instant,
};

mod kmeans;
use kmeans::{KMeans, Point};
use png::Transformations;

fn read_image(path: &Path) -> Result<(Vec<Point>, u32, u32), std::io::Error> {
    let image = File::open(path)?;

    let mut decoder = png::Decoder::new(image);
    decoder.set_transformations(Transformations::normalize_to_color8());

    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];

    let info = reader.next_frame(&mut buf).unwrap();
    let width = info.width;
    let height = info.height;

    let bytes = &buf[..info.buffer_size()];
    let len = bytes.len();

    let (color, depth) = reader.output_color_type();
    println!("Color: {:?}, Depth: {:?}", color, depth);
    println!(
        "Image bytes len: {len}, line_size: {line_size}, width: {width}, height: {height}",
        line_size = info.line_size
    );

    const COL_DIM: usize = 4; // R-G-B-A sequence
    assert!(len % COL_DIM == 0);

    let points = (0..len)
        .step_by(COL_DIM)
        .map(|ind| {
            Point {
                x: (bytes[ind + 0] as f32).into(),
                y: (bytes[ind + 1] as f32).into(),
                z: (bytes[ind + 2] as f32).into(),
                // Ignore apha
            }
        })
        .collect();

    Ok((points, width, height))
}

fn write_image(
    image_points: Vec<Point>,
    (width, height): (u32, u32),
    path: &Path,
) -> Result<(), std::io::Error> {
    assert!(width * height == image_points.len() as u32);

    if let Some(dirs) = path.parent() {
        create_dir_all(dirs)?
    }

    let file = File::create(path)?;
    let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::Rgb);

    let mut writer = encoder.write_header()?;
    let data = image_points
        .into_iter()
        .flat_map(|point| {
            [
                point.x.round() as u8,
                point.y.round() as u8,
                point.z.round() as u8,
            ]
        })
        .collect::<Vec<_>>();

    writer.write_image_data(&data)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);

    let filepath = Path::new(&args[1]);
    let k = args[2].parse::<usize>().expect("Wrong K");

    let filename = filepath.file_name().expect("Could not get filename");
    let file_extension = filepath.extension().expect("Could not get file extension");

    println!("Filename: {filename:?}");

    let (image_points, width, height) = read_image(filepath).map_err(|err| {
        eprintln!("Could not read an image {filepath:?}: {err}");
        err
    })?;
    println!("Colors count: {count}", count = image_points.len());

    let unique_points = HashSet::<Point>::from_iter(image_points.clone().into_iter())
        .into_iter()
        .collect::<Vec<_>>();
    println!("Unique colors count: {count}", count = unique_points.len());

    let kmeans = {
        let now = Instant::now();

        let kmeans = KMeans::generate_k_means(unique_points, k);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        kmeans
    };

    let image_points = image_points
        .into_iter()
        .map(|point| kmeans.cluster_point_for_point(&point))
        .collect::<Vec<_>>();

    let out_filename =
        Path::new(filename).join(format!("{:03}.{}", k, file_extension.to_str().unwrap()));
    let out_filepath = Path::new("out").join(out_filename);
    let out_filepath = out_filepath.as_path();

    let _ = write_image(image_points, (width, height), out_filepath).map_err(|err| {
        eprintln!("Could not write an image: {err}");
        err
    })?;

    Ok(())
}

// cargo run -- assets/images/img_512.png 16
