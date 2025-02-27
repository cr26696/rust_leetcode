use std::collections::HashSet;

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let h = matrix.len();
    let w = matrix[0].len();
    let mut set_z_col: HashSet<usize> = HashSet::new();
    let mut set_z_row: HashSet<usize> = HashSet::new();
    let mut zero_pos:Vec<(usize,usize)>=Vec::new();
    for i in 0..h{
        for j in 0..w{
            if matrix[i][j] == 0{
                set_z_row.insert(i);
                set_z_col.insert(j);
            }
        }
    }
    let set_z_col:Vec<usize> = set_z_col.into_iter().collect();
    let set_z_row:Vec<usize> = set_z_row.into_iter().collect();

    for col in set_z_col{
        for idx in 0..h{
            matrix[idx][col] = 0;
        }
    }
    for row in set_z_row {
        let temp = vec![0;w];
        matrix[row] = temp;
    }

}

fn main() {
    let matrix1 = vec![[0,1,2,0],[3,4,5,2],[1,3,1,5]];
    let mut matrix: Vec<Vec<i32>> = vec![];
    for e in matrix1{
        matrix.push(e.to_vec());
    }
    set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}
