// https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&page=show_problem&problem=1402
use std::io::{self,Read};
use std::collections:VecDeque;
fn main(){
let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut case_no =1;

    loop {
        let v:usize =it.next().unwrap().parse().unwrap();
                let e: usize = it.next().unwrap().parse().unwrap();

        if v==0 && e==0 {
            break;
        }
        let mut time = vec![0i64; v+1];
        for i in 1..=v{
            time[i] = it.next().unwrap().parse().unwrap();
        }

let mut adj= vec![Vec::new(); v+1];
        let mut radj = vec![Vec::new(); v+1];
        let mut indeg = vec![0usize; v+1];
for - in 0..e{
             let x: usize = it.next().unwrap().parse().unwrap();
            let y: usize = it.next().unwrap().parse().unwrap();
adj[x].push(y);
            radj[y].push(x);
            indeg[y] +=1;
        }


let q = VecDeque::new();
        for i in 1..v {
            if indeg[i] ==0 {
                q.push_back(i);
            }
        }
        let mut topo = Vec::new();
        while let Some(u) = q.pop_front() {
            topo.push(u);
            for &nxt in &adj[u] {
                indeg[nxt] -= 1;
                if indeg[nxt] == 0 {
                    q.push_back(nxt);
                }
            }
        }

    }
}
