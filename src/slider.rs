use crate::rgenerator;

fn permute_row( board: &mut Vec<Vec<u32>>, index: usize, direction: i8 )
{
    assert!( direction == 1 || direction == -1, "Board must be permuted by only 1 or -1 ");
    // get row size
    let len: i8 = board[index].len() as i8;
    let (start,end) = if direction == 1 { (0,len-1) } else { (len-1,0) };
    let cached_element = board[index][start as usize];
    let mut i = start;

    loop {
        if i == end {
            board[index][i as usize] = cached_element;
            break;
        } else {
            board[index][i as usize] = board[index][ (i+ direction) as usize];
        }
        i += direction;
    }
}

fn permute_column( board: &mut Vec<Vec<u32>>, index: usize , direction: i8 )
{
    assert!( direction == 1 || direction == -1, "Board must be permuted by only 1 or -1 ");
    // get row size
    let len: i8 = board.len() as i8;
    let (start,end) = if direction == 1 { (0,len-1) } else { (len-1,0) };
    let cached_element = board[start as usize][index];
    let mut i = start;

    loop {
        if i == end {
            board[i as usize][index] = cached_element;
            break;
        } else {
            board[i as usize][index] = board[(i+ direction) as usize][index];
        }
        i += direction;
    }
}

pub fn generate_puzzle( index: usize, num_iterations: u32, rng: &mut rgenerator::Generator ) -> Vec<Vec<u32>>
{
    let mut shuffled_board: Vec<Vec<u32>> = fetch_target(index);

    println!("{:?}", shuffled_board);

    for _i in 0..num_iterations 
    {
        if rng.int(2) == 0 {
            let row_length: u32 = shuffled_board[0].len() as u32;
            permute_row( &mut shuffled_board, rng.int(row_length) as usize , 1 );
        } else {
            let col_length: u32 = shuffled_board.len() as u32;
            permute_column( &mut shuffled_board, rng.int(col_length) as usize , 1 );
        }
    }
    println!("{:?}", shuffled_board);

    return shuffled_board;
}

pub fn verify_puzzle( index: usize, shuffled_board: &mut Vec<Vec<u32>>, solution: &Vec<u8>) -> bool
{
    /* Solution formatting is explained in the readme */
    let target_board: &Vec<Vec<u32>> = &fetch_target(index);
    let direction: [i8;2] = [1,-1];

    // solution must be even
    if solution.len()%2 == 1 {
        return false;
    }
    
    for i in solution.chunks(2) {

        let direction_and_type = i[0] as usize;
        let pos = i[1] as usize;
        
        // we need to do these checks to make sure we dont get an oob error and panic
        // Alternate option is to use .get() and return a result
        if direction_and_type > 3 {
            false;
        }
        let max_index = if direction_and_type < 2 { target_board[0].len() } else { target_board.len() }; 
        if pos >= max_index {
            return false;
        }

        if direction_and_type < 2 {
            permute_row(shuffled_board, pos , direction[direction_and_type] );
        } else {
            permute_column(shuffled_board, pos , direction[direction_and_type-2]);
        }
    }

    return shuffled_board == target_board;
}


pub fn fetch_target( index: usize ) -> Vec<Vec<u32>> {
    /* Very hacky solution - only appropiate because of small vector sizes 
       if larger puzzles were needed transition to file based approach */
    assert!( index < 7 , "Attemping to fetch non-existant puzzle index ");
    match index {
        0 => vec![vec![2,5,3],vec![4,6,4],vec![1,5,0]],
        1 => vec![vec![3,4,2],vec![5,6,5],vec![1,4,0]],
        2 => vec![vec![1,4,1],vec![2,6,5],vec![2,3,2]],
        3 => vec![vec![2,4,6,4],vec![3,6,3,3],vec![1,4,2,3]],
        4 => vec![vec![2,7,2,1],vec![4,3,3,4],vec![4,3,3,4]],
        5 => vec![vec![0,0,1,1,2],vec![0,0,1,1,2],vec![1,1,0,2,2]],
        6 => vec![vec![2,3,6,3,4],vec![4,1,3,2,0],vec![5,1,3,4,6]],
        _ => vec![vec![]]
    }
}
