use quickraw::{data, DemosaicingMethod, Input, Output, OutputType};

fn main() {
    use quickraw::Export;

    let raw_data = std::fs::read("sample.dng").unwrap();

    // This will not panic
    let (_thumbnail_data, _orientation) = Export::export_thumbnail_data(&raw_data).unwrap();

    // This will not panic
    quickraw::Export::export_thumbnail_to_file("sample.dng", "sample.thumbnail.jpg").unwrap();

    // This will panic
    let export_job = Export::new(
        Input::ByFile("sample.dng"),
        Output::new(
            DemosaicingMethod::SuperPixel,
            data::XYZ2SRGB,
            data::GAMMA_SRGB,
            OutputType::Raw16,
            true,
            true,
        ),
    ).unwrap();

    export_job.export_image(10).unwrap();

}
