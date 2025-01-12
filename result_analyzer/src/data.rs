use std::{
    fs::File,
    io::{Error, Read},
    num::{ParseFloatError, ParseIntError},
    ops::Deref,
    path::Path,
};

#[derive(Debug)]
pub enum DataError {
    IoError(Error),
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
    MissingValueError,
}

impl From<Error> for DataError {
    fn from(error: Error) -> Self {
        DataError::IoError(error)
    }
}

impl From<ParseFloatError> for DataError {
    fn from(error: ParseFloatError) -> Self {
        DataError::ParseFloatError(error)
    }
}

impl From<ParseIntError> for DataError {
    fn from(error: ParseIntError) -> Self {
        DataError::ParseIntError(error)
    }
}

pub trait TimeRange {
    fn get_time_range(&self) -> (f32, f32);
}

pub fn get_time_range(time_ranges: &[Box<dyn TimeRange>]) -> (f32, f32) {
    time_ranges
        .iter()
        .fold((0.0, 0.0), |(old_min, old_max), val| {
            let (new_min, new_max) = val.get_time_range();

            (f32::min(new_min, old_min), f32::max(new_max, old_max))
        })
}

#[derive(Debug)]
pub enum BenchMarkVariant {
    Small,
    Large,
}

impl TryFrom<&str> for BenchMarkVariant {
    type Error = DataError;

    fn try_from(variant_str: &str) -> Result<Self, Self::Error> {
        match variant_str {
            "Large" => Ok(BenchMarkVariant::Large),
            "Small" => Ok(BenchMarkVariant::Small),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct BenchMark {
    pub name: String,
    pub variant: BenchMarkVariant,
    pub data: BenchMarkData,
}

impl TryFrom<&Path> for BenchMark {
    type Error = DataError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if !path.is_file() {
            todo!()
        }

        let name = path
            .file_name()
            .ok_or(DataError::MissingValueError)?
            .to_string_lossy();

        let mut name_components = name.split('_').skip(2);

        let data = BenchMarkData::try_from(path)?;

        let variant = BenchMarkVariant::try_from(
            name_components.next().ok_or(DataError::MissingValueError)?,
        )?;

        let name: String = name_components.collect();
        let name: String = String::from(&name[..name.len() - 4]);

        Ok(BenchMark {
            name,
            variant,
            data,
        })
    }
}

impl TimeRange for BenchMark {
    fn get_time_range(&self) -> (f32, f32) {
        self.data.get_time_range()
    }
}

#[derive(Debug)]
pub enum BenchMarkData {
    Memory(MemoryData),
    Request(RequestData),
}

impl TimeRange for BenchMarkData {
    fn get_time_range(&self) -> (f32, f32) {
        match self {
            BenchMarkData::Memory(data) => data.get_time_range(),
            BenchMarkData::Request(data) => data.get_time_range(),
        }
    }
}

impl TryFrom<&Path> for BenchMarkData {
    type Error = DataError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if !path.is_file() {
            todo!()
        }

        let name = path
            .file_name()
            .ok_or(DataError::MissingValueError)?
            .to_string_lossy();

        let mut name_components = name.split('_');

        let benchmark_type = name_components.next().ok_or(DataError::MissingValueError)?;

        return match benchmark_type {
            "memory" => Ok(BenchMarkData::Memory(MemoryData::try_from(path)?)),
            "request" => Ok(BenchMarkData::Request(RequestData::try_from(path)?)),
            _ => panic!(),
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryTimeStamp {
    pub timestamp: f32,
    pub kb: f32,
}

#[derive(Debug, Clone)]
pub struct MemoryData(Vec<MemoryTimeStamp>);

impl MemoryData {
    pub fn new() -> Self {
        MemoryData(Vec::new())
    }
}

impl TimeRange for MemoryData {
    fn get_time_range(&self) -> (f32, f32) {
        self.iter().fold(
            (0.0, 0.0),
            |(old_min, old_max), MemoryTimeStamp { timestamp, kb: _ }| {
                (f32::min(*timestamp, old_min), f32::max(*timestamp, old_max))
            },
        )
    }
}

impl TryFrom<&Path> for MemoryData {
    type Error = DataError;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let mut file = File::open(file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data_points: Vec<MemoryTimeStamp> = contents
            .split('\n')
            .skip(1)
            .map(|line: &str| -> Result<MemoryTimeStamp, DataError> {
                let mut component = line.split(",");

                let timestamp: f32 = component
                    .next()
                    .ok_or(DataError::MissingValueError)?
                    .trim()
                    .parse()?;

                let kb: f32 = component
                    .next()
                    .ok_or(DataError::MissingValueError)?
                    .trim()
                    .parse()?;

                Ok(MemoryTimeStamp { timestamp, kb })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // add ordering

        Ok(MemoryData(data_points))
    }
}

impl Deref for MemoryData {
    type Target = Vec<MemoryTimeStamp>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestTimeStamp {
    pub start_timestamp: f32,
    pub duration: f32,
    pub response: u32,
}

#[derive(Debug, Clone)]
pub struct RequestData(Vec<RequestTimeStamp>);

impl RequestData {
    pub fn new() -> Self {
        RequestData(Vec::new())
    }
}

impl TimeRange for RequestData {
    fn get_time_range(&self) -> (f32, f32) {
        self.iter().fold(
            (0.0, 0.0),
            |(old_min, old_max),
             RequestTimeStamp {
                 start_timestamp,
                 duration,
                 response: _,
             }| {
                (
                    f32::min(*start_timestamp, old_min),
                    f32::max(*start_timestamp + *duration, old_max),
                )
            },
        )
    }
}
impl TryFrom<&Path> for RequestData {
    type Error = DataError;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let mut file = File::open(file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data_points: Vec<RequestTimeStamp> = contents
            .split('\n')
            .skip(1)
            .map(|line: &str| -> Result<RequestTimeStamp, DataError> {
                let mut component = line.split(",");

                let start_timestamp: f32 = component
                    .next()
                    .ok_or(DataError::MissingValueError)?
                    .trim()
                    .parse()?;

                let response: u32 = component
                    .next()
                    .ok_or(DataError::MissingValueError)?
                    .trim()
                    .parse::<u32>()?;

                let duration: f32 = component
                    .next()
                    .ok_or(DataError::MissingValueError)?
                    .trim()
                    .parse()?;

                Ok(RequestTimeStamp {
                    start_timestamp,
                    response,
                    duration,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // add ordering

        Ok(RequestData(data_points))
    }
}

impl Deref for RequestData {
    type Target = Vec<RequestTimeStamp>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
