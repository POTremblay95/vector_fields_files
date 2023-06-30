use vec_field_files::*;

fn main() {
    let vec1: Vec<&[f32; 3]> = vec![&[0.0, 0.0, 0.0]; 10];

    vect_to_fga(&vec1, FgaFileParameter::default())
        .expect("There was an error when converting the vector of fga!");
}
