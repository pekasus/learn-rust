/// c --> c - 'A' (or c as u32 - 'A' as u32) --> square side 1 + 2*(c - 'A')
/// n-th row (starting from 0):
/// 0-th and n-1-th row: only 1 letter 'A' at the center
/// n-th row: spaces... L.. L..
pub fn get_diamond(c: char) -> Vec<String> {
    let highest_char_position = (c as u32 - 'A' as u32) as usize;
    let row_length = highest_char_position*2 + 1;
    
    let top_rows: Vec<_> = (0..=highest_char_position).map(|char_index| {
        let mut row = vec![' '; row_length];
        let letter = char::from_u32('A' as u32 + char_index as u32).unwrap();
        row[highest_char_position - char_index] = letter;
        row[highest_char_position + char_index] = letter;
        row.iter().collect::<String>()
    }).collect();

    let mut bottom_rows = top_rows.iter().rev();
    bottom_rows.next();
    
    bottom_rows.fold(top_rows.clone(), |mut acc, row| {
        acc.push(row.clone());
        acc
    })
   
}
