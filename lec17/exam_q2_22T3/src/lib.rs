#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum DiffResult<'left, 'right> {
    LeftOnly(&'left str),
    RightOnly(&'right str),
    Both(&'left str)
}

pub fn compare_rolls<'left, 'right>(left_roll: &'left str, right_roll: &'right str) -> Vec<DiffResult<'left, 'right>> {
    let mut results = Vec::new();

    for left_name in left_roll.split('\n') {
        let appears_in_right = right_roll.split('\n')
            .find(|right_name| left_name == *right_name)
            .is_some();
        
        results.push(if appears_in_right {
            DiffResult::Both(left_name)
        } else {
            DiffResult::LeftOnly(left_name)
        });
    }

    for right_name in right_roll.split('\n') {
        let appears_in_left = left_roll.split('\n')
            .find(|left_name| right_name == *left_name)
            .is_some();

        if !appears_in_left {
            results.push(DiffResult::RightOnly(right_name));
        }
    }

    results.sort();

    results
}
