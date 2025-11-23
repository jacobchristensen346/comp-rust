// Write a function that tranposes a n x n matrix.
// This is a modification of the function
// which could only transpose 3x3 matrices.

fn transpose<const R: usize, const C: usize>(matrix: [[i32; C]; R]) -> [[i32; C]; R] {
    let mut trans_matrix = matrix; // Initialize to og matrix 
    //for idx in 0..=2 {
    //    trans_matrix[0][idx] = matrix[idx][0];
    //    trans_matrix[1][idx] = matrix[idx][1];
    //    trans_matrix[2][idx] = matrix[idx][2];
    //}
    //return trans_matrix;
    
    // You could also use a nested loop...
    for i in 0..=R-1 {
        for j in 0..=C-1 {
            trans_matrix[i][j] = matrix[j][i];
        }
    }
    return trans_matrix;
}

fn main() {
    let matrix = [
        [101, 102, 103, 104], // <-- the comment makes rustfmt add a newline
        [201, 202, 203, 204],
        [301, 302, 303, 304],
        [401, 402, 403, 404]
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
