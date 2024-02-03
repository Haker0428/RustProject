use super::*;

/// #Usage:
/// ```ignore
/// let filname = PathBuf::from("./input/challenge.csv");
/// let csv_data = load_csv(filname).unwrap();
/// let modifyed_data = replace_colunmn(csv_data, "City", "Beijing").unwrap();
/// let output_file = write_csv(&modifyed_data, "output/test.csv");
/// assert!(output_file.is_ok());
/// ```
pub fn replace_colunmn(data: String, column:&str, replacement:&str)
    -> Result<String, Error> {
    let mut lines = data.lines();
    let headers = lines.next().unwrap();
    let columns : Vec<&str> = headers.split(',').collect();
    let column_number = columns.iter().position(|&e| {
        println!("{:?} {}", e, column);
        e.trim() == column
    });
    
    let column_number = match column_number {
        Some(column) => column,
        None => Err("column name doesn't exist in the input file {}")?
    };

    let mut result = String::with_capacity(data.capacity());
    result.push_str(&columns.join(","));
    result.push('\n');

    for line in lines {
        let mut records : Vec<&str> = line.split(',').collect();
        records[column_number] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }

    Ok(result)
}