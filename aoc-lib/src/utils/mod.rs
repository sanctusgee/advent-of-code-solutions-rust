pub mod input;
pub mod output;
pub mod numbers;
// Re-export commonly used items
pub use input::{
    download_input, ensure_input, get_input_path, load_input, load_input_lines,
    parse_lines, parse_lines_with_delimiter, is_in_sorted_ranges, 
    merge_u64_ranges, parse_ranges_generic,
};
pub use output::SolutionOutput;
pub use numbers::num_digits;

