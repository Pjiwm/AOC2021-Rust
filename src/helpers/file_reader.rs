use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    num::ParseIntError,
};

pub fn read(path: isize) -> Result<Vec<String>, Error> {
    let file = File::open(format!("src/static/{}", path))?;
    let br = BufReader::new(file);
    let mut v = Vec::<String>::new();
    for line in br.lines() {
        let line = line?;
        let n = line.trim();
        v.push(n.to_owned());
    }
    Ok(v)
}

pub fn string_to_i32(vec: Vec<String>) -> Result<Vec<i32>, ParseIntError> {
    let mut int_vec = Vec::<i32>::new();
    for v in vec {
        let i = v.parse::<i32>()?;
        int_vec.push(i);
    }
    Ok(int_vec)
}
