use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::ptr::null;
use crate::pathfinder::algorithms::BaseAlgorithm::BaseAlgorithm;
use crate::pathfinder::grid::Grid;

struct AStar{
}

impl AStar{
    fn heuristic(cell: [i64; 2], end_node: [i64;2]) -> i64{
        (cell[0] - end_node[1]).abs() + (cell[1] - end_node[1]).abs()
    }
}
impl BaseAlgorithm for AStar {
    fn solve(mut grid: Grid, diagonal_allowed: bool) -> Vec<[i64;2]>{
        let start = grid.get_start();
        let end = grid.get_end();
        let directions: &[(i64, i64)];

        let mut g_costs: HashMap<[i64; 2], i64> = HashMap::new();
        let mut f_costs: HashMap<[i64;2], i64> = HashMap::new();
        let mut came_from: HashMap<[i64;2], [i64;2]> = HashMap::new();
        let mut open_list: BinaryHeap<(Reverse<i64>, Vec<i64>)> = BinaryHeap::new();

        if (start == [-1,-1] || end == [-1,-1]){
            return Vec::new();
        }
        

        if diagonal_allowed {
            directions = &[
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1), (0, 1),
                (1, -1), (1, 0), (1, 1)
            ]
        } else {
            directions = &[(-1, 0), (0, -1), (0, 1), (1, 0)]
        }

        g_costs.insert(start, 0);
        f_costs.insert(start, Self::heuristic(start, end));

        while !open_list.is_empty() {
            let current_cell = open_list.pop().unwrap();

            if !current_cell.1.eq(&start){
                grid.update_field(current_cell.1[0] as usize, current_cell.1[1] as usize, 3)
            } else if current_cell.1.eq(&end){
                break;
            }

            for (dx, dy) in directions {
                let current_cell_copy = current_cell.clone();
                let new_cell = [current_cell_copy.1[0] + dx, current_cell_copy.1[1] + dy];
                if grid.is_valid_grid_cell(new_cell[0], new_cell[1]) && !came_from.contains_key(&new_cell){
                    let state = grid.get_state_of_field_cord(new_cell[0] as usize, new_cell[1] as usize);

                    if (state != 3){
                        let g_cost = g_costs[&[current_cell.1[0], current_cell.1[1]]] + 1;
                        if !g_costs.contains_key(&new_cell) || g_cost < g_costs[&[new_cell[0], new_cell[1]]]{
                            came_from.insert(new_cell, [current_cell_copy.1[0], current_cell_copy.1[1]]);
                            g_costs.insert(new_cell, g_cost);
                            f_costs.insert(new_cell, Self::heuristic(new_cell, end));

                                if open_list.iter().any(|(_, v)| v == &new_cell){
                                    open_list.push((
                                        Reverse(*f_costs.get(&new_cell).unwrap()),
                                        vec![new_cell[0], new_cell[1]]
                                    ));
                                }
                        }
                    }
                }
            }
        }
        let mut path: Vec<[i64;2]> = Vec::new();
        let mut current_node = end;

        while let Some(parent) = came_from.get(&current_node) {
            if current_node == start {
                break;
            }

            path.push(current_node);
            current_node = *parent;
        }

        path
    }
}