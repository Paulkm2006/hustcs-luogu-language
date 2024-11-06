fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut s = input.split_whitespace();
    let m: usize = s.next().unwrap().parse().unwrap();
    let n: i32 = s.next().unwrap().parse().unwrap();
    let mut map = vec![vec![-1; m]; m];
    for _ in 0..n{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut s = input.split_whitespace();
        let x: usize = s.next().unwrap().parse().unwrap();
        let y: usize = s.next().unwrap().parse().unwrap();
        let c: i32 = s.next().unwrap().parse().unwrap();
        map[x - 1][y - 1] = c;
    }
    let mut f = vec![vec![255; m]; m];
    let c = map[0][0];
    dfs(&mut map, &mut f, 0, 0, m as i32,c, 0, true);
    println!("{}", match f[m-1][m-1]{
        255 => -1,
        _ => f[m-1][m-1]});
}

fn dfs(
    map: &mut Vec<Vec<i32>>,
    f: &mut Vec<Vec<i32>>,
    x: i32,
    y: i32,
    m: i32,
    c: i32,
    now: i32,
    can_use_magic: bool,
) {
    let d: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    f[x as usize][y as usize] = now;
    for i in d.iter() {
        let nx = x + i[0];
        let ny = y + i[1];
        if nx < 0 || nx >= m || ny < 0 || ny >= m {
            continue;
        }
        let cost = f[nx as usize][ny as usize];
        if now+(c-map[nx as usize][ny as usize]).abs() < cost && map[nx as usize][ny as usize] != -1 {
            dfs(map, f, nx, ny, m, map[nx as usize][ny as usize], now+(c-map[nx as usize][ny as usize]).abs(), true);
        }
        if can_use_magic {
            if now + 2 < f[nx as usize][ny as usize] && map[nx as usize][ny as usize] == -1 {
                dfs(map, f, nx, ny, m, c, now + 2, false);
            }
        }
    }
}