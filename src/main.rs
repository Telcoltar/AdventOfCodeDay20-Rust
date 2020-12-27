mod test_solutions;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

fn get_input_data(filename: &str) -> HashMap<i32, Vec<Vec<i32>>>  {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut tiles: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
    let mut is_new_tile = true;
    let mut num: i32 = 0;
    let mut current_tile: Vec<Vec<i32>> = vec![];

    for line in f.lines() {
        let line = line.unwrap();
        if line.trim() == "" {
            tiles.insert(num, current_tile.clone());
            is_new_tile = true;
            continue
        }
        if is_new_tile {
            let num_split = &line[5..];
            num = num_split.replace(":","").parse().unwrap();
            current_tile = Vec::new();
            is_new_tile = false;
            continue
        }
        current_tile.push(line.chars().map(|s| if s == '#' {1} else {0}).collect())
    }
    tiles.insert(num, current_tile.clone());
    return tiles

}

fn get_left_border(tile: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut border: Vec<i32> = Vec::new();
    for line in tile {
        border.push(line[0])
    }
    return border;
}

fn get_right_border(tile: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut border: Vec<i32> = Vec::new();
    for line in tile {
        border.push(line[line.len() - 1])
    }
    return border;
}

fn add_code_to_dict(dict: &mut HashMap<Vec<i32>, Vec<i32>>,code: Vec<i32>, num: i32) {
    let mut rev_code = code.clone();
    rev_code.reverse();
    let current_nums = dict.get_mut(&code);
    if current_nums.is_some() {
        current_nums.unwrap().push(num)
    } else {
        dict.insert(code, vec![num]);
    }
    let current_nums = dict.get_mut(&rev_code);
    if current_nums.is_some() {
        current_nums.unwrap().push(num)
    } else {
        dict.insert(rev_code, vec![num]);
    }
}

fn get_borders(tile: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    return vec![tile[0].clone(), get_right_border(tile),
                tile[tile.len() - 1].clone(), get_left_border(tile)]
}

fn build_code_to_tile_dict(tiles: &HashMap<i32, Vec<Vec<i32>>>) -> HashMap<Vec<i32>, Vec<i32>> {
    let mut dict: HashMap<Vec<i32>, Vec<i32>>  = HashMap::new();
    for (num, tile) in tiles {
        for border in get_borders(tile) {
            add_code_to_dict( &mut dict, border, *num);
        }
    }
    return dict;
}

fn build_neighbor_dict(tiles: &HashMap<i32, Vec<Vec<i32>>>) -> HashMap<i32, Vec<i32>> {
    let mut dict: HashMap<i32, Vec<i32>> = HashMap::new();
    let code_to_tile = build_code_to_tile_dict(tiles);
    for (num, tile) in tiles {
        for border in get_borders(tile) {
            let tile_nums = code_to_tile.get(&border).unwrap();
            let mut neighbor_num;
            if tile_nums.len() == 2 {
                neighbor_num = tile_nums[0];
                if neighbor_num == *num {
                    neighbor_num = tile_nums[1];
                }
            } else {
                neighbor_num = -1;
            }
            let neighbor_list = dict.get_mut(num);
            if neighbor_list.is_some() {
                neighbor_list.unwrap().push(neighbor_num);
            } else {
                dict.insert(*num, vec![neighbor_num]);
            }
        }
    }
    return dict;
}

fn classify_tiles(tiles: &HashMap<i32, Vec<Vec<i32>>>) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let neighbor_dict = build_neighbor_dict(tiles);
    let mut corners: Vec<i32> = Vec::new();
    let mut edges: Vec<i32> = Vec::new();
    let mut middle: Vec<i32> = Vec::new();
    for (num, neighbors) in neighbor_dict {
        let neighbor_count = neighbors.iter().filter(|&n| *n == -1).count();
        if neighbor_count == 2 {
            corners.push(num);
        } else if neighbor_count == 1 {
            edges.push(num);
        } else {
            middle.push(num);
        }
    }
    return (corners, edges, middle);
}

fn solution_part_1(filename: &str) -> i64 {
    let tiles = get_input_data(filename);
    let (corners, _edges, _middle) = classify_tiles(&tiles);
    return corners.iter().map(|&el| el as i64).product();
}

fn orient_tile(neighbor_tiles: &mut Vec<i32>, constrains: Vec<(usize, i32)>) -> (i32, i32) {
    for i in 0..4 {
        if constrains.iter().all(|&t| neighbor_tiles[t.0] == t.1) {
            return (i, 0);
        }
        let tmp = neighbor_tiles.pop().unwrap();
        neighbor_tiles.insert(0,tmp);
    }

    let tmp_tile = neighbor_tiles[1];
    neighbor_tiles[1] = neighbor_tiles[3];
    neighbor_tiles[3] = tmp_tile;

    for i in 0..4 {
        if constrains.iter().all(|&t| neighbor_tiles[t.0] == t.1) {
            return (i, 1);
        }
        let tmp = neighbor_tiles.pop().unwrap();
        neighbor_tiles.insert(0,tmp);
    }

    return (-1, 0)
}

fn build_grid(tiles: &HashMap<i32, Vec<Vec<i32>>>) -> Vec<Vec<(i32, (i32, i32))>> {
    let (corners, _edges, _middle) = classify_tiles(tiles);
    let mut neighbor_dict = build_neighbor_dict(tiles);
    let mut grid: Vec<Vec<(i32, (i32, i32))>> = Vec::new();
    let mut current_tile = corners[0];
    let mut prev_tile = -1;
    let mut current_row: Vec<(i32, (i32, i32))> = Vec::new();
    while current_tile != -1 {
        current_row.push((current_tile,
                          (orient_tile(neighbor_dict.get_mut(&current_tile).unwrap(),
                                       vec![(0, -1), (3, prev_tile)]))));
        prev_tile = current_tile;
        current_tile = neighbor_dict[&current_tile][1];
        debug!("{:?}, {:?}, {:?}", prev_tile, current_tile, neighbor_dict[&prev_tile]);
    }
    let height = current_row.len();
    grid.push(current_row);
    for i in 1..height {
        let mut current_row: Vec<(i32, (i32, i32))> = Vec::new();
        let mut j = 0;
        current_tile = neighbor_dict[&grid[i-1][0].0][2];
        prev_tile = -1;
        while current_tile != -1 {
            current_row.push((current_tile,
                              (orient_tile(neighbor_dict.get_mut(&current_tile).unwrap(),
                                           vec![(0, grid[i-1][j].0), (3, prev_tile)]))));
            prev_tile = current_tile;
            current_tile = neighbor_dict[&current_tile][1];
            debug!("{:?}, {:?}, {:?}", prev_tile, current_tile, neighbor_dict[&prev_tile]);
            j += 1;
        }
        grid.push(current_row);
    }
    debug!("{:?}", grid);
    return grid;
}

fn rotate_90d(tile: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut rotated_tile: Vec<Vec<i32>> = Vec::new();
    let mut current_row;
    for i in 0..tile.len() {
        current_row = Vec::new();
        for j in 0..tile[0].len() {
            current_row.push(tile[tile[0].len() - j - 1][i]);
        }
        rotated_tile.push(current_row)
    }
    return rotated_tile;
}

fn flip_tile(tile: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut flipped_tile: Vec<Vec<i32>> = Vec::new();
    let mut current_row;
    for i in 0..tile.len() {
        current_row = Vec::new();
        for j in 0..tile[0].len() {
            current_row.push(tile[i][tile[0].len() - j - 1]);
        }
        flipped_tile.push(current_row)
    }
    return flipped_tile;
}

fn orient_tiles(tiles: &mut HashMap<i32, Vec<Vec<i32>>>, grid: &Vec<Vec<(i32, (i32, i32))>>) {
    for line in grid {
        for (num, or) in line {
            debug!("{:?}, {:?}", num, or);
            if or.1 == 1 {
                tiles.insert(*num, flip_tile(&tiles[num]));
            }
            for _i in 0..or.0 {
                tiles.insert(*num, rotate_90d(&tiles[num]));
            }
        }
    }
}

fn get_sea_monster() -> Vec<(usize, usize)> {
    let f = File::open("sea_monster.txt").unwrap();
    let f = BufReader::new(f);

    let mut monster_grid: Vec<Vec<i32>> = Vec::new();

    for line in f.lines() {
        let mut current_line: Vec<i32> = Vec::new();
        for c in line.unwrap().chars() {
            if c == '#' {
                current_line.push(1);
            } else {
                current_line.push(0);
            }
        }
        monster_grid.push(current_line);
    }

    let mut monster: Vec<(usize, usize)> = Vec::new();
    for (i, line) in monster_grid.iter().enumerate() {
        for (j, point) in line.iter().enumerate() {
            if point == &1 {
                monster.push((j, i));
            }
        }
    }
    return monster;
}

fn is_monster(grid: &Vec<Vec<i32>>, monster: &Vec<(usize, usize)>, x: usize, y: usize) -> bool {
    for point in monster {
        if grid[point.1 + y - 1][point.0 + x] != 1 {
            return false;
        }
    }
    return true;
}

fn find_monster(grid: &Vec<Vec<i32>>, monster: &Vec<(usize, usize)>) -> i32 {
    let mut count = 0;
    let monster_width = 19 as usize;
    let monster_height = 2 as usize;
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    for i in 1..(grid_height - monster_height + 1) {
        for j in 0..(grid_width - monster_width) {
            if grid[i][j] == 1 {
                if is_monster(grid, monster, j, i) {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn find_monsters(mut grid: Vec<Vec<i32>>) -> i32 {
    let monster = get_sea_monster();
    let mut count: i32;
    for _ in 0..4 {
        count = find_monster(&grid, &monster);
        if count != 0 {
            return count;
        }
        grid = rotate_90d(&grid);
    }
    grid = flip_tile(&grid);
    for _ in 0..4 {
        count = find_monster(&grid, &monster);
        if count != 0 {
            return count;
        }
        grid = rotate_90d(&grid);
    }
    return 0;
}

fn pprint_grid(grid: &Vec<Vec<(i32, (i32, i32))>>) -> String {
   let mut rep: String = String::new();
   rep.push_str("\n");
   for line in grid {
       for (point, _or) in line {
           rep.push_str(&point.to_string());
           rep.push_str(" ");
       }
       rep.push_str("\n");
   }
    return rep;
}

fn solution_part_2(filename: &str) -> i32{
    let mut tiles = get_input_data(filename);
    debug!("{:?}", classify_tiles(&tiles));
    let grid = build_grid(&tiles);
    orient_tiles(&mut tiles, &grid);
    let tile_height = tiles[tiles.keys().nth(0).unwrap()].len();
    let mut result_grid: Vec<Vec<i32>> = Vec::new();
    for (i, line) in grid.iter().enumerate() {
        for _ in 0..(tile_height - 2) {
            let new_row: Vec<i32> = Vec::new();
            result_grid.push(new_row)
        }
        for (num,_or) in line {
            for (j, row) in tiles[num].iter().enumerate() {
                if j == 0 || j == tile_height -1 {
                    continue;
                }
                result_grid[i * (tile_height - 2) + (j - 1)].extend(&row[1..(tile_height - 1)]);
            }
        }
    }
    let count_hash = &result_grid.iter().map(|l| l.iter().sum::<i32>()).sum::<i32>();
    let sea_monster_count = find_monsters(result_grid);
    debug!("{}", pprint_grid(&grid));
    debug!("{:?}", sea_monster_count);
    debug!("{:?}", count_hash);
    return count_hash - sea_monster_count * 15;
}

fn main() {
    env_logger::init();
    info!("{}", solution_part_1("inputData.txt"));
    info!("{}", solution_part_2("inputData.txt"));
}
