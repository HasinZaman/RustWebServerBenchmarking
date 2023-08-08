use std::{ops::Deref, path::Path, fs::File, io::Read};

#[derive(Debug)]
pub enum DataError {
    IoError(std::io::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
    MissingValueError,
}

impl From<std::io::Error> for DataError {
    fn from(error: std::io::Error) -> Self {
        DataError::IoError(error)
    }
}

impl From<std::num::ParseFloatError> for DataError {
    fn from(error: std::num::ParseFloatError) -> Self {
        DataError::ParseFloatError(error)
    }
}

impl From<std::num::ParseIntError> for DataError {
    fn from(error: std::num::ParseIntError) -> Self {
        DataError::ParseIntError(error)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryTimeStamp {
    pub timestamp: f32,
    pub kb: f32
}

#[derive(Debug, Clone)]
pub struct MemoryData(Vec<MemoryTimeStamp>);

impl MemoryData {
    pub fn new() -> Self {
        MemoryData(Vec::new())
    }
}

impl TryFrom<&Path> for MemoryData {
    type Error = DataError;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let mut file = File::open(file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data_points: Vec<MemoryTimeStamp> = contents.split('\n')
            .skip(1)
            .map(
                |line: &str| -> Result<MemoryTimeStamp, DataError> {
                    let mut component = line.split(",");

                    let timestamp: f32 = component.next()
                        .ok_or(DataError::MissingValueError)?
                        .trim()
                        .parse()?;

                    let kb: f32 = component.next()
                        .ok_or(DataError::MissingValueError)?
                        .trim()
                        .parse()?;

                    Ok(MemoryTimeStamp {
                        timestamp,
                        kb,
                    })
                }
            )
            .collect::<Result<Vec<_>, _>>()?;


        // add ordering

        Ok(MemoryData(data_points))
    }
}

impl Deref for MemoryData{
    type Target = Vec<MemoryTimeStamp>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestTimeStamp{
    pub start_timestamp: f32,
    pub duration: f32,
    pub response: u32
}

#[derive(Debug, Clone)]
pub struct RequestData(Vec<RequestTimeStamp>);

impl RequestData {
    pub fn new() -> Self {
        RequestData(Vec::new())
    }
}

impl TryFrom<&Path> for RequestData {
    type Error = DataError;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let mut file = File::open(file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data_points: Vec<RequestTimeStamp> = contents.split('\n')
            .skip(1)
            .map(
                |line: &str| -> Result<RequestTimeStamp, DataError> {
                    let mut component = line.split(",");

                    let start_timestamp: f32 = component.next()
                        .ok_or(DataError::MissingValueError)?
                        .trim()
                        .parse()?;

                    let response: u32 = component.next()
                        .ok_or(DataError::MissingValueError)?
                        .trim()
                        .parse::<u32>()?;

                    let duration: f32 = component.next()
                        .ok_or(DataError::MissingValueError)?
                        .trim()
                        .parse()?;

                    Ok(RequestTimeStamp {
                        start_timestamp,
                        response,
                        duration,
                    })
                }
            )
            .collect::<Result<Vec<_>, _>>()?;


        // add ordering

        Ok(RequestData(data_points))
    }
}

impl Deref for RequestData{
    type Target = Vec<RequestTimeStamp>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}