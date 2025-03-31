use std::collections::{HashMap, HashSet, VecDeque};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greedy_snake_move(param_body: Vec<i32>, param_fruit: Vec<i32>) -> i32 
{
    let mut obstacle: HashSet<[i32; 2]> = HashSet::new();
    let mut searched: HashSet<[i32; 2]> = HashSet::new();
    let mut pred: HashMap<[i32; 2], [i32; 2]> = HashMap::new();
    obstacle.insert([param_body[2], param_body[3]]);

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
    use super::*;

    // 基础移动方向测试
    #[test]
    fn basic_movement() {
        // 水果在正上方
        assert_eq!(greedy_snake_move(vec![5,5,5,4,5,3,5,2], vec![5,6]), 0);
        
        // 水果在正左方
        assert_eq!(greedy_snake_move(vec![5,5,6,5,7,5,8,5], vec![4,5]), 1);
        
        // 水果在正下方
        assert_eq!(greedy_snake_move(vec![5,5,5,6,5,7,5,8], vec![5,3]), 2);
        
        // 水果在正右方
        assert_eq!(greedy_snake_move(vec![5,5,4,5,3,5,2,5], vec![6,5]), 3);
    }

    // 边界条件测试
    #[test]
    fn boundary_conditions() {
        // 上边界移动
        assert_eq!(greedy_snake_move(vec![5,8,5,7,5,6,5,5], vec![5,4]), 1);

        // 左边界移动
        assert_eq!(greedy_snake_move(vec![1,5,2,5,3,5,4,5], vec![5,5]), 0);
        
        // 下边界移动
        assert_eq!(greedy_snake_move(vec![5,1,5,2,5,3,5,4], vec![5,5]), 1);
        
        // 右边界移动
        assert_eq!(greedy_snake_move(vec![8,5,7,5,6,5,5,5], vec![4,5]), 0);
    }

    // 路径搜索测试
    #[test]
    fn pathfinding() {
        // 盘曲在角落（左下角），头尾相接
        assert_eq!(greedy_snake_move(vec![1,1,2,1,2,2,1,2], vec![5,5]), 0);

        // 右下角
        assert_eq!(greedy_snake_move(vec![8,1,7,1,7,2,8,2], vec![5,5]), 0);
        
        // 左上角
        assert_eq!(greedy_snake_move(vec![1,8,2,8,2,7,1,7], vec![5,5]), 2);

        // 右上角
        assert_eq!(greedy_snake_move(vec![8,8,7,8,7,7,8,7], vec![5,5]), 2);
        
        // 长路径
        assert_eq!(greedy_snake_move(vec![2,1,1,1,1,2,2,2], vec![8,8]), 0);
    }
}