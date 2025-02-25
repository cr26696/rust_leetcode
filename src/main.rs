use std::mem::swap;


pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len  = matrix.len();
    let mut cnt = 0;
    while len - cnt*2 > 1{
        for i in cnt..len-cnt{
            let des = len-cnt -i -1;
            let mut temp;
            temp = matrix[cnt][i];
            matrix[cnt][i] = matrix[des][cnt];
            matrix[des][cnt] = matrix[len-cnt][des];
            matrix[len-cnt][des] = matrix[cnt][des];
            matrix[cnt][des] = temp;
        }
        cnt +=1;
    }
    return;
}

fn main() {
    let matrix1 = vec![[1,2,3],[4,5,6],[7,8,9]];
    let mut matrix: Vec<Vec<i32>> = vec![];
    for e in matrix1{
        matrix.push(e.to_vec());
    }
    let r = rotate(&mut matrix);
    println!("{:?}", r);
}
