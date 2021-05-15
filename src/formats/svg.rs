use svgcleaner::cleaner;
use svgcleaner::{CleaningOptions, ParseOptions, WriteOptions};
use svgdom::WriteBuffer;

pub fn convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = load(input)?;
    let data = apply(&data)?;
    save(output, &data)
}

pub fn load(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = cleaner::load_file(path)?;
    Ok(data)
}

pub fn apply(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut data = cleaner::parse_data(input, &ParseOptions::default())?;

    let write_options = write_options();
    let cleaning_options = cleaning_options();

    cleaner::clean_doc(&mut data, &cleaning_options, &write_options)?;

    let mut buffer = Vec::new();
    data.write_buf_opt(&write_options, &mut buffer);
    Ok(buffer)
}

pub fn save(path: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    cleaner::save_file(data, path)?;
    Ok(())
}

fn write_options() -> WriteOptions {
    let mut write_options = WriteOptions::default();

    write_options.use_single_quote = true;
    write_options.trim_hex_colors = true;
    write_options.write_hidden_attributes = false;
    write_options.remove_leading_zero = true;
    write_options.use_compact_path_notation = true;
    write_options.join_arc_to_flags = true;
    write_options.remove_duplicated_path_commands = true;
    write_options.use_implicit_lineto_commands = true;
    write_options.simplify_transform_matrices = true;

    write_options
}

fn cleaning_options() -> CleaningOptions {
    let mut cleaning_options = CleaningOptions::default();

    cleaning_options.remove_unused_defs = true;
    cleaning_options.convert_shapes = true;
    cleaning_options.remove_title = true;
    cleaning_options.remove_desc = true;
    cleaning_options.remove_metadata = true;
    cleaning_options.remove_dupl_linear_gradients = true;
    cleaning_options.remove_dupl_radial_gradients = true;
    cleaning_options.remove_dupl_fe_gaussian_blur = true;
    cleaning_options.ungroup_groups = true;
    cleaning_options.ungroup_defs = true;
    cleaning_options.group_by_style = true;
    cleaning_options.merge_gradients = true;
    cleaning_options.regroup_gradient_stops = true;
    cleaning_options.remove_invalid_stops = true;
    cleaning_options.remove_invisible_elements = true;
    cleaning_options.resolve_use = true;

    cleaning_options.remove_version = true;
    cleaning_options.remove_unreferenced_ids = true;
    cleaning_options.trim_ids = true;
    cleaning_options.remove_text_attributes = true;
    cleaning_options.remove_unused_coordinates = true;
    cleaning_options.remove_default_attributes = true;
    cleaning_options.remove_xmlns_xlink_attribute = true;
    cleaning_options.remove_needless_attributes = true;
    cleaning_options.remove_gradient_attributes = true;
    cleaning_options.apply_transform_to_gradients = true;
    cleaning_options.apply_transform_to_shapes = true;

    cleaning_options
}
