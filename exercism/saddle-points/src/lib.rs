use std::collections::HashMap;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // let mut result = Vec::new();

    let mut row_candidates: HashMap<usize, Vec<usize>> = HashMap::new(); // row_index => column_indexes of highest value

    input.iter().enumerate().for_each(|(row_index, row)| {
        // 1. find the maximum value in this row
        let max_value = row.iter().max();
        let candidate_col_ids = if max_value.is_none() {
            vec![]
        } else {
            let max_value = *max_value.unwrap();

            // 2. collect the indices for that maximum
            //let mut column_indices_for_max = <Vec<(usize, usize)>>::new();
            row.iter()
                .enumerate()
                .filter(|(_idx, &value)| value == max_value)
                .map(|(idx, _value)| idx)
                .collect()
        };
        row_candidates.insert(row_index, candidate_col_ids);
        //row_candidates[row_index] = column_indices_for_max;
    });

    let mut column_candidates = HashMap::new(); // col index => row indexes of the lowest value
    (0..input[0].len()).for_each(|col_idx| {
        let min_value = input.iter().map(|row| row[col_idx]).min().unwrap();
        let candidate_row_ids: Vec<_> = input
            .iter()
            .enumerate()
            .filter(|(_row_idx, row)| row[col_idx] == min_value)
            .map(|(row_idx, _value)| row_idx)
            .collect();
        column_candidates.insert(col_idx, candidate_row_ids);
    });

    row_candidates.iter().fold(
        Vec::<(usize, usize)>::new(),
        |mut acc, (&row_idx, candidates)| {
            candidates.iter().for_each(|&col_idx| {
                let potential_rows = column_candidates.get(&col_idx).unwrap();
                if potential_rows.contains(&row_idx) {
                    acc.push((row_idx, col_idx));
                }
            });

            acc
        },
    )

    // result
}
