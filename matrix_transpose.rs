// Write a function that tranposes a 3x3 matrix.

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut trans_matrix = matrix; // Initialize to og matrix 
    //for idx in 0..=2 {
    //    trans_matrix[0][idx] = matrix[idx][0];
    //    trans_matrix[1][idx] = matrix[idx][1];
    //    trans_matrix[2][idx] = matrix[idx][2];
    //}
    //return trans_matrix;
    
    // You could also use a nested loop...
    for i in 0..=2 {
        for j in 0..=2 {
            trans_matrix[i][j] = matrix[j][i];
        }
    }
    return trans_matrix;
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original:");
    for row in &matrix {
        println!("{:?}", row);
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in &transposed {
        println!("{:?}", row);
    }
}
