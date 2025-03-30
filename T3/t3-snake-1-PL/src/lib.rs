use std::collections::{HashMap, HashSet, VecDeque};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greedy_snake_step(n: i32, snake: Vec<i32>, snake_num: i32, other_snakes: Vec<i32>, food_num: i32, foods: Vec<i32>, round: i32) -> i32 
{
    let mut hard_obstacle: HashSet<[i32; 2]> = HashSet::new();
    let mut soft_obstacle: HashSet<[i32; 2]> = HashSet::new();
    let mut searched: HashSet<[i32; 2]> = HashSet::new();
    let mut pred: HashMap<[i32; 2], [i32; 2]> = HashMap::new();
    let mut food_pos: HashSet<[i32; 2]> = HashSet::new();
    for i in 0..food_num as usize
    {
        food_pos.insert([foods[i * 2], foods[i * 2 + 1]]);
    }
    hard_obstacle.insert([snake[2], snake[3]]);
    for i in 0..snake_num as usize
    {
        hard_obstacle.insert([other_snakes[i * 8], other_snakes[i * 8 + 1]]);
        hard_obstacle.insert([other_snakes[i * 8 + 2], other_snakes[i * 8 + 3]]);
        hard_obstacle.insert([other_snakes[i * 8 + 4], other_snakes[i * 8 + 5]]);
    }
    for i in 0..snake_num as usize
    {
        soft_obstacle.insert([other_snakes[i * 8], other_snakes[i * 8 + 1]+1]);
        soft_obstacle.insert([other_snakes[i * 8]-1, other_snakes[i * 8 + 1]]);
        soft_obstacle.insert([other_snakes[i * 8], other_snakes[i * 8 + 1]-1]);
        soft_obstacle.insert([other_snakes[i * 8]+1, other_snakes[i * 8 + 1]]);
    }
    let mut up_flag=false;
    let mut left_flag=false;
    let mut down_flag=false;
    let mut right_flag=false;
    if soft_obstacle.contains(&[snake[0], snake[1]+1]) && !hard_obstacle.contains(&[snake[0], snake[1]+1]) {
        up_flag=true;
    }
    if soft_obstacle.contains(&[snake[0]-1, snake[1]]) && !hard_obstacle.contains(&[snake[0]-1, snake[1]]) {
        left_flag=true;
    }
    if soft_obstacle.contains(&[snake[0], snake[1]-1]) && !hard_obstacle.contains(&[snake[0], snake[1]-1]) {
        down_flag=true;
    }
    if soft_obstacle.contains(&[snake[0]+1, snake[1]]) && !hard_obstacle.contains(&[snake[0]+1, snake[1]])  {
        right_flag=true;
    }
    soft_obstacle.clear();
    if up_flag {
        soft_obstacle.insert([snake[0], snake[1]+1]);
    }
    if left_flag {
        soft_obstacle.insert([snake[0]-1, snake[1]]);
    }
    if down_flag {
        soft_obstacle.insert([snake[0], snake[1]-1]);
    }
    if right_flag {
        soft_obstacle.insert([snake[0]+1, snake[1]]);
    }
    let mut queue: VecDeque<[i32; 2]> = VecDeque::new();
    queue.push_back([snake[0], snake[1]]);
    searched.insert([snake[0], snake[1]]);

    let mut reachable = false;
    let mut cur_pos = [-1, -1];
    while let Some([x, y]) = queue.pop_front()
    {
        if y != n && !hard_obstacle.contains(&[x, y + 1]) && !searched.contains(&[x, y + 1]) && !soft_obstacle.contains(&[x, y + 1])
        {
            queue.push_back([x, y + 1]);
            searched.insert([x, y + 1]);
            pred.insert([x, y + 1], [x, y]);
            if food_pos.contains(&[x, y + 1])
            {
                cur_pos = [x, y + 1];
                reachable = true;
                break;
            }
        }
        if x != 1 && !hard_obstacle.contains(&[x - 1, y]) && !searched.contains(&[x - 1, y]) && !soft_obstacle.contains(&[x - 1, y])
        {
            queue.push_back([x - 1, y]);
            searched.insert([x - 1, y]);
            pred.insert([x - 1, y], [x, y]);
            if food_pos.contains(&[x - 1, y])
            {
                cur_pos = [x - 1, y];
                reachable = true;
                break;
            }
        }
        if y != 1 && !hard_obstacle.contains(&[x, y - 1]) && !searched.contains(&[x, y - 1]) && !soft_obstacle.contains(&[x, y - 1])
        {
            queue.push_back([x, y - 1]);
            searched.insert([x, y - 1]);
            pred.insert([x, y - 1], [x, y]);
            if food_pos.contains(&[x, y - 1])
            {
                cur_pos = [x, y - 1];
                reachable = true;
                break;
            }
        }
        if x != n && !hard_obstacle.contains(&[x + 1, y]) && !searched.contains(&[x + 1, y]) && !soft_obstacle.contains(&[x + 1, y])
        {
            queue.push_back([x + 1, y]);
            searched.insert([x + 1, y]);
            pred.insert([x + 1, y], [x, y]);
            if food_pos.contains(&[x + 1, y])
            {
                cur_pos = [x + 1, y];
                reachable = true;
                break;
            }
        }
    }
    
    if !reachable 
    {
        let mut up_flag_hard=hard_obstacle.contains(&[snake[0], snake[1]+1]);
        let mut left_flag_hard=hard_obstacle.contains(&[snake[0]-1, snake[1]]);
        let mut down_flag_hard=hard_obstacle.contains(&[snake[0], snake[1]-1]);
        let mut right_flag_hard=hard_obstacle.contains(&[snake[0]+1, snake[1]]);
        //朝既没有硬障碍又没有软障碍的方向移动
        if !up_flag && !up_flag_hard && snake[1] != n
        {
            return 0;
        }
        if !left_flag && !left_flag_hard && snake[0] != 1
        {
            return 1;
        }
        if !down_flag && !down_flag_hard && snake[1] != 1
        {
            return 2;
        }
        if !right_flag && !right_flag_hard && snake[0] != n
        {
            return 3;
        }
        //朝没有硬障碍的方向移动
        let mut selectivity=up_flag_hard as u8+left_flag_hard as u8+down_flag_hard as u8+right_flag_hard as u8;
        if selectivity==4
        {
            //四个方向都有硬障碍
            return 0;
        } else if selectivity==3 {   
            //三个方向都有硬障碍
            if !up_flag_hard
            {
                return 0;
            }
            if !left_flag_hard
            {
                return 1;
            }
            if !down_flag_hard
            {
                return 2;
            } else {
                return 3;
            }
        } else {
            //两个方向或以下有障碍物，此时可选
            if !up_flag_hard && snake[1] != n
            {
                return 0;
            }
            if !left_flag_hard && snake[0] != 1
            {
                return 1;
            }
            if !down_flag_hard && snake[1] != 1
            {
                return 2;
            }
            if !right_flag_hard && snake[0] != n
            {
                return 3;
            }
            return 0; 
        }
    }
    else 
    {
        while let Some(&prev_pos) = pred.get(&cur_pos) 
        {
            if prev_pos[0] == snake[0] && prev_pos[1] == snake[1] 
            {
                break;
            }
            cur_pos = prev_pos;    
        }

        if cur_pos[0] == snake[0] && cur_pos[1] == snake[1] + 1
        {
            return 0;
        }
        else if cur_pos[0] == snake[0] - 1 && cur_pos[1] == snake[1]
        {
            return 1;
        }
        else if cur_pos[0] == snake[0] && cur_pos[1] == snake[1] - 1
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
