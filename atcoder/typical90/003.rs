use proconio::input;

fn dfs(
    list: &Vec<Vec<usize>>,
    distances: &mut Vec<usize>,
    checked: &mut Vec<bool>,
    start_node: usize,
) -> bool {
    for (_index, edge) in list[start_node].iter().enumerate() {
        // println!("index = {}, edge = {}", _index, edge);
        if checked[*edge] == false {
            distances[*edge] = distances[start_node] + 1;
            checked[*edge] = true;
            dfs(list, distances, checked, *edge);
        }
    }
    return true;
}

// 木構造の直径を求める
fn diameter(list: &Vec<Vec<usize>>) -> usize {
    // 始点0からの各頂点の距離を格納する
    let start_node: usize = 0;
    let mut distances: Vec<usize> = vec![0; list.len()];
    let mut checked: Vec<bool> = vec![false; list.len()];
    distances[start_node] = 0;
    checked[start_node] = true;

    // println!("distances {:?}", distances);
    dfs(list, &mut distances, &mut checked, start_node);
    // println!("distances {:?}", distances);

    // 配列内の最大値を確認し，その値をもつインデックスを新たな始点とする．
    let (start_node, &_max_value): (usize, &usize) = distances
        .iter()
        .enumerate()
        .max_by_key(|(_, &x): &(usize, &usize)| -> usize { x })
        .unwrap();
    // println!("max {} {}", start_node, _max_value);

    // 再初期化
    for index in 0..distances.len() {
        distances[index] = 0;
        checked[index] = false;
    }
    distances[start_node] = 0;
    checked[start_node] = true;
    // println!("distances {:?}", distances);

    dfs(list, &mut distances, &mut checked, start_node);
    // println!("distances {:?}", distances);
    let result = *(distances.iter().max().unwrap());
    return result + 1;
}

fn main() {
    input! {
        n: usize,
        edges: [[usize; 2]; n - 1],
    }
    // println!("path {:?}", path);
    let mut list: Vec<Vec<usize>> = Vec::with_capacity(n);
    // println!("before {:?}", list);
    for _i in 0..n {
        let edges: Vec<usize> = Vec::new();
        list.push(edges);
    }
    for i in 0..n - 1 {
        list[edges[i][0] - 1].push(edges[i][1] - 1);
        list[edges[i][1] - 1].push(edges[i][0] - 1);
    }
    // println!("after {:?}", list);
    println!("{}", diameter(&list));
}
