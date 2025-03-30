use std::collections::{HashMap, HashSet, VecDeque};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(param_body: Vec<i32>, param_fruit: Vec<i32>, param_barrier: Vec<i32>) -> i32 
{
    let mut obstacle: HashSet<[i32; 2]> = HashSet::new();
    let mut searched: HashSet<[i32; 2]> = HashSet::new();
    let mut pred: HashMap<[i32; 2], [i32; 2]> = HashMap::new();
    obstacle.insert([param_body[2], param_body[3]]);
    for i in 0..12
    {
        obstacle.insert([param_barrier[i * 2], param_barrier[i * 2 + 1]]);
    }

    let mut queue: VecDeque<[i32; 2]> = VecDeque::new();
    queue.push_back([param_body[0], param_body[1]]);
    searched.insert([param_body[0], param_body[1]]);

    let mut reachable = false;
    while let Some([x, y]) = queue.pop_front()
    {
        if y != 8 && !obstacle.contains(&[x, y + 1]) && !searched.contains(&[x, y + 1])
        {
            queue.push_back([x, y + 1]);
            searched.insert([x, y + 1]);
            pred.insert([x, y + 1], [x, y]);
            if x == param_fruit[0] && y + 1 == param_fruit[1] 
            {
                reachable = true;
                break;
            }
        }
        if x != 1 && !obstacle.contains(&[x - 1, y]) && !searched.contains(&[x - 1, y])
        {
            queue.push_back([x - 1, y]);
            searched.insert([x - 1, y]);
            pred.insert([x - 1, y], [x, y]);
            if x - 1 == param_fruit[0] && y == param_fruit[1]
            {
                reachable = true;
                break;
            }
        }
        if y != 1 && !obstacle.contains(&[x, y - 1]) && !searched.contains(&[x, y - 1])
        {
            queue.push_back([x, y - 1]);
            searched.insert([x, y - 1]);
            pred.insert([x, y - 1], [x, y]);
            if x == param_fruit[0] && y - 1 == param_fruit[1]
            {
                reachable = true;
                break;
            }
        }
        if x != 8 && !obstacle.contains(&[x + 1, y]) && !searched.contains(&[x + 1, y])
        {
            queue.push_back([x + 1, y]);
            searched.insert([x + 1, y]);
            pred.insert([x + 1, y], [x, y]);
            if x + 1 == param_fruit[0] && y == param_fruit[1]
            {
                reachable = true;
                break;
            }
        }
    }
    
    if !reachable 
    {
        return -1; 
    }
    else 
    {
        let mut cur_pos = [param_fruit[0], param_fruit[1]];
        while let Some(&prev_pos) = pred.get(&cur_pos) 
        {
            if prev_pos[0] == param_body[0] && prev_pos[1] == param_body[1] 
            {
                break;
            }
            cur_pos = prev_pos;    
        }

        if cur_pos[0] == param_body[0] && cur_pos[1] == param_body[1] + 1
        {
            return 0;
        }
        else if cur_pos[0] == param_body[0] - 1 && cur_pos[1] == param_body[1]
        {
            return 1;
        }
        else if cur_pos[0] == param_body[0] && cur_pos[1] == param_body[1] - 1
        {
            return 2;
        }
        else 
        {
            return 3;    
        }  
    }
}

#[cfg(test)]
mod tests {

}
