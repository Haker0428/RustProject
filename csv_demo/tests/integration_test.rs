#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use csv_demo::{
        Opt,
        {load_csv, write_csv},
        replace_colunmn,
    };

    #[test]
    fn test_csv_challenge() {
        let filname = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filname).unwrap();
        let modifyed_data = replace_colunmn(csv_data, "City", "Beijing").unwrap();
        let output_file = write_csv(&modifyed_data, "output/test.csv");
        assert!(output_file.is_ok());
    }
}