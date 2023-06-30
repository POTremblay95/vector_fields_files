use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct FgaFileParameter {
    name: String,        //Path included if not in working directory
    resolution: [u8; 3], //Resolution in x,y and z
    min_axes: [i32; 3],  //Minimum bounding box for each coord
    max_axes: [i32; 3],  //Maximum bounding box for each coord
}

impl Default for FgaFileParameter {
    fn default() -> Self {
        Self {
            name: String::from("vectors.fga"),
            resolution: [10, 10, 10],
            min_axes: [-100, -100, -100],
            max_axes: [100, 100, 100],
        }
    }
}

pub fn vec_to_string<T: Copy + Display>(vector: &[T; 3]) -> String {
    format!("{0}, {1}, {2}\n", vector[0], vector[1], vector[2])
}

pub fn vectors_to_string(vectors: &[&[f32; 3]]) -> String {
    let mut data = String::from("");

    for &vec in vectors {
        data.push_str(vec_to_string(vec).as_str());
    }

    data
}

pub fn vect_to_fga(vectors: &[&[f32; 3]], file_param: FgaFileParameter) -> std::io::Result<()> {
    let mut data = String::from(format!("{0}", vec_to_string(&file_param.resolution)));
    data.push_str(format!("{0}", vec_to_string(&file_param.min_axes)).as_str());
    data.push_str(format!("{0}", vec_to_string(&file_param.max_axes)).as_str());
    data.push_str(vectors_to_string(vectors).as_str());

    let mut file = File::create(file_param.name)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn hello() {
    println!("Hello World! It works!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
