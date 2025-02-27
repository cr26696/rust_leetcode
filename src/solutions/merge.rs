use std::vec;


pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() == 1{return intervals};
    let mut out: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
    intervals.sort_unstable_by(|a,b| a.get(0).unwrap().cmp(b.get(0).unwrap()));
    
    
    let mut l = intervals[0][0];//区间起点
    let mut r = intervals[0][1];//区间终点
    let mut i = 1;//指向下个考虑的区间

    'outer: while i < intervals.len(){
        //
        while intervals[i][0] == l{
            //考虑所有的起点相同的区间
            r= r.max(intervals[i][1]);
            i+=1;
            if i == intervals.len(){
                out.push(vec![l,r]);
                break 'outer;
            }
        }
        //尝试与下个区间合并
        if intervals[i][0] <= r{
            r = intervals[i][1].max(r);
            i+=1;
            if i == intervals.len(){
                out.push(vec![l,r]);
                break 'outer;
            }
        }else{
            out.push(vec![l,r]);
            l = intervals[i][0];
            r = intervals[i][1];
            i+=1;
            if i == intervals.len(){
                out.push(vec![l,r]);
                break 'outer;
            }
        }
    }
    out
}
fn main() {
    let intervals1 = vec![[1,4],[2,3]];
    let mut intervals: Vec<Vec<i32>> = vec![];
    for e in intervals1{
        intervals.push(e.to_vec());
    }
    let r = merge(intervals);
    println!("{:?}", r);
}
