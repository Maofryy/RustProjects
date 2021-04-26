use csv;
use std::collections::HashMap;
use std::fs;
use std::io;

fn read_file(filename: String) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

pub fn read_csv(filename: String) -> Result<HashMap<String, String>, io::Error> {
    let csv = read_file(filename)?;

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut tcp_dict = HashMap::new();
    for record in reader.records() {
        let record = record?;
        tcp_dict.insert(record[1].to_string(), record[2].to_string());
    }

    Ok(tcp_dict)
}

#[cfg(test)]
mod tests {
    use super::read_file;

    #[test]
    fn test_read() {
        assert!(read_file("data/tcp.csv".to_string()).is_ok());
        assert!(read_file("data/top.csv".to_string()).is_err());
    }
}
