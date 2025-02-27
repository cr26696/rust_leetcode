

enum Dir {
    Left,
    Right,
    Up,
    Down
}
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut top = 0 as usize;//边界位置：最多可抵达的索引位置
    let mut bottom = matrix.len()-1;
    let mut left: usize = 0;
    let mut right = matrix[0].len()-1;
    let mut out:Vec<i32> = Vec::new();
    out.push(matrix[0][0]);
    let mut dir:Dir;
    if bottom == 0&&right==0{
        return out;
    }else if right == 0{
        dir = Dir::Down;
    }else{
        dir = Dir::Right;
    }

    let (mut x,mut y):(usize,usize) = (0,0);//行-列 号
    'outer:loop{
        match dir {
            Dir::Right=>{
                loop {
                    x+=1;
                    out.push(matrix[y][x]);
                    if x==right{
                        if y==bottom{break 'outer}
                        top +=1;
                        dir = Dir::Down;
                        break;
                    }
                }
            },
            Dir::Down=>{
                loop {
                    y+=1;
                    out.push(matrix[y][x]);
                    if y==bottom{
                        if x==left{break 'outer}
                        right -=1;
                        dir = Dir::Left;
                        break;
                    }
                }
            },
            Dir::Left=>{
                loop {
                    x-=1;
                    out.push(matrix[y][x]);
                    if x==left{
                        if y==top{break 'outer}
                        bottom -=1;
                        dir = Dir::Up;
                        break;
                    }
                }
            },
            Dir::Up=>{
                loop {
                    y-=1;
                    out.push(matrix[y][x]);
                    if y==top{
                        if x==right{break 'outer}
                        left +=1;
                        dir = Dir::Right;
                        break;
                    }
                }
            },
        }
    }
    out
}
fn main() {
    let matrix1 = vec![[1,2,3,4],[5,6,7,8],[9,10,11,12]];
    let mut matrix: Vec<Vec<i32>> = vec![];
    for e in matrix1{
        matrix.push(e.to_vec());
    }
    let r = spiral_order(matrix);
    println!("{:?}", r);
}
