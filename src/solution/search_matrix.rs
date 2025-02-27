use std::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if target > *matrix.last().unwrap().last().unwrap()
        || target < *matrix.first().unwrap().first().unwrap()
    {
        return false;
    }
    let (m, n) = (matrix.len(), matrix[0].len()); //m行n列
    let mut dir_right = true;
    let mut pos: (usize, usize) = (0, 0); //next check
    'outer: loop {
        if matrix[pos.0][pos.1] == target {
            return true;
        }

        if dir_right && pos.1 + 1 == n {
            //下一行
        } else if !dir_right && pos.1 == 0 {
            //下一行
        } else {
            if dir_right {
                pos.1 += 1;
                if matrix[pos.0][pos.1] <= target {continue;}
            } else {
                pos.1 -= 1;
                if matrix[pos.0][pos.1] >= target {continue;}
            }
        }

        //下一行 下一行存在判断
        if pos.0 + 1 == m {
            break 'outer;
        } else {
            pos.0 += 1;
            match target.cmp(&matrix[pos.0][pos.1]) {
                Ordering::Less => {
                    dir_right = false;
                }
                Ordering::Greater => {
                    dir_right = true;
                }
                _ => {
                    return true;
                }
            }
        }
    }
    return false;
}

fn main() {
    let matrix1 = vec![[1],[3],[5]];
    let mut matrix: Vec<Vec<i32>> = vec![];
    for e in matrix1 {
        matrix.push(e.to_vec());
    }
    let target = 5;
    let r = search_matrix(matrix, target);
    println!("{:?}", r);
}
