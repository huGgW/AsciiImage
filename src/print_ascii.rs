pub fn print(ascii_mat: &Vec<Vec<char>>) {
    for row in ascii_mat {
        // for make it square, twice per char 
        for c in row {
            print!("{}{}", c, c);
        }
        println!();
    }
}