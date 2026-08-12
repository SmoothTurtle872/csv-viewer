use rfd::AsyncFileDialog;

pub async fn open_file() -> Result<File, FileError> {
    let selected = AsyncFileDialog::new()
        .add_filter("CSV Files", &["csv"])
        .set_directory("./")
        .pick_file()
        .await;
    match selected {
        None => Err(FileError::NoFileFound),
        Some(handle) => {
            let mut reader = csv::Reader::from_path(handle.path())?;
            let mut records = Vec::new();
            records.push(reader.headers()?.iter().map(|s| s.to_string()).collect());

            for record in reader.records() {
                let record = record?;
                let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                records.push(row);
            }
            Ok(records.into())
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileError {
    NoFileFound,
    /// Although this means pattern matching isn't possible, as clone is not implemented, the error message is returned instead
    CSVError(String),
}

impl From<csv::Error> for FileError {
    fn from(value: csv::Error) -> Self {
        Self::CSVError(value.to_string())
    }
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFileFound => {
                write!(f, "File Not Found")
            }
            Self::CSVError(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct File(Vec<Vec<String>>);

impl File {
    pub fn get_data(&self) -> &Vec<Vec<String>> {
        &self.0
    }

    pub fn get_size(&self) -> [usize; 2] {
        if self.0.len() < 1 {
            [0, 0]
        } else {
            [self.0.len(), self.0[0].len()]
        }
    }
}

impl From<Vec<Vec<String>>> for File {
    fn from(value: Vec<Vec<String>>) -> Self {
        Self(value)
    }
}
