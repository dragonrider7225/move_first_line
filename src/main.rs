use std::{
    convert::TryFrom,
    env,
    error::Error,
    fmt::{self, Display, Formatter},
    fs::OpenOptions,
    io::{self, Read, Write},
};

const EMPTY_LINE: [u8; 1] = [b'\n'];

#[derive(Debug)]
pub enum MainError {
    IoError(io::Error),
    TryFromIntError(std::num::TryFromIntError),
}

impl Display for MainError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MainError::IoError(ref e) => write!(f, "{}", e),
            MainError::TryFromIntError(ref e) => write!(f, "{}", e),
        }
    }
}

impl Error for MainError {
}

impl From<io::Error> for MainError {
    fn from(base: io::Error) -> MainError {
        MainError::IoError(base)
    }
}

impl From<std::num::TryFromIntError> for MainError {
    fn from(base: std::num::TryFromIntError) -> MainError {
        MainError::TryFromIntError(base)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let _ = args.next();
    let from_filename = args.next().expect("Must have file to read from");
    let to_filename = args.next().expect("Must have file to write to");
    let mut from_file = OpenOptions::new().read(true).open(&from_filename)?;
    let mut contents = Vec::with_capacity(
        usize::try_from(from_file.metadata()?.len())?);
    let _ = from_file.read_to_end(&mut contents)?;
    let mut split = contents.splitn(2, |&c| c == b'\n');
    let first_line = split.next().unwrap_or(&EMPTY_LINE);
    let rest = split.next().unwrap_or(&EMPTY_LINE);
    let mut to_file = OpenOptions::new().write(true).create_new(true)
        .open(&to_filename)?;
    to_file.write_all(first_line)?;
    // split in impl<T> Deref<Target=[T]> for Vec<T> excludes the split token,
    // so the line terminator needs to be manually added back in.
    to_file.write_all(&EMPTY_LINE)?;
    let mut from_file = OpenOptions::new().write(true).truncate(true)
        .open(&from_filename)?;
    from_file.write_all(rest)?;
    Ok(())
}
