impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
           
        let mut triangle: Vec<Vec<i32>> = Vec::new();
        
        if num_rows == 0 {
            
            return triangle;
        }
        
          triangle.push(vec![1]);

        
    for i in 1..num_rows as usize{
        
    let mut row: Vec<i32> = Vec::new();
    let prev_row = &triangle[i - 1];    
        row.push(1);
        
        for j in 1..i{
            row.push(prev_row[j-1] + prev_row[j]);
        }
                row.push(1);
        triangle.push(row);

        
        }
        triangle
    }
}