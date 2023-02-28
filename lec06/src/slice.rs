fn longest_run_without(list: &[i32], without: i32) -> &[i32] {
    let mut current_run_length = 0;
    let mut longest_run_length = 0;
    let mut longest_start_index = 0;

    for (index, elem) in list.iter().enumerate() {
        if *elem != without {
            current_run_length += 1;
            if current_run_length > longest_run_length {
                longest_run_length = current_run_length;
                longest_start_index = index - (current_run_length - 1);
            }
        } else {
            current_run_length = 0;
        }
    }

    &list[longest_start_index..(longest_start_index + longest_run_length)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 3, 5, 6];
        assert_eq!(longest_run_without(&list, 3), &[4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn empty_list() {
        let list = [];
        assert_eq!(longest_run_without(&list, 42), &[]);
    }

    #[test]
    fn whole_list() {
        let list = vec![1, 2, 3, 4, 5];
        assert_eq!(longest_run_without(&list, 42), &[1, 2, 3, 4, 5]);
    }
}