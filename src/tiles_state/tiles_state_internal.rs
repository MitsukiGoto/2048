use super::TilesState;
use rand::Rng;

trait ArrayExt {}

impl TilesState {
    pub(crate) fn initialise_tile(&mut self) {
        let y = self.random_coordinate();
        let x = self.random_coordinate();
        if self.is_tile_initialised(y, x) {
            self.initialise_tile()
        }
        let new_value = self.generate_new_tile_value();
        self.update_certain_tile_internal(y, x, new_value);
    }

    pub(crate) fn is_tile_initialised(&self, y: i32, x: i32) -> bool {
        self.game_state[y as usize][x as usize] != 0
    }

    pub(crate) fn generate_new_tile_value(&mut self) -> i32 {
        if self.randomiser.gen_range(1..10) == 1 {
            4
        } else {
            2
        }
    }

    pub(crate) fn move_tiles_internal(&self, array: [i32; 4]) -> [i32; 4] {
        let arr = Self::merge_matching_pair(array);
        Self::move_after_merge(arr)
    }

    fn merge_matching_pair(mut arr: [i32; 4]) -> [i32; 4] {
        arr.reverse();
        let mut from = 0;
        while from < 4 {
            let from_value = arr[from];
            if from_value != 0 {
                const NOT_FOUND: usize = 10;
                let option = arr
                    .iter()
                    .skip(from + 1)
                    .enumerate()
                    .fold(Some(NOT_FOUND), |opt, (i, x)| -> Option<usize> {
                        if opt.is_none() || opt.unwrap() != NOT_FOUND {
                            return opt;
                        }
                        if *x == 0 {
                            return opt;
                        } else {
                            return if *x == from_value {
                                Some(from + i + 1)
                            } else {
                                None
                            };
                        }
                    })
                    .and_then(|x| -> Option<usize> {
                        return if x != NOT_FOUND { Some(x) } else { None };
                    });
                if let Some(to) = option {
                    arr[from] *= 2;
                    arr[to] = 0;
                    from += 1;
                }
            }
            from += 1;
        }
        arr.reverse();
        return arr;
    }

    fn move_after_merge(mut arr: [i32; 4]) -> [i32; 4] {
        arr.reverse();
        for i in 0..4 {
            if arr[i] == 0 {
                // create new array that skips first 'i' value
                // i.e. when 'i' = 1 then [2,0,0,4] => [0,0,4]
                // thus the index of the new array is equivalent
                // to what is added 'i+1' to the old one since
                // the index counts start at 0.
                if let Some(j) = arr.iter().skip(i + 1).position(|&x| x != 0) {
                    let equivalent_index = i + j + 1;
                    arr.swap(i, equivalent_index);
                    arr[equivalent_index] = 0;
                }
            }
        }
        arr.reverse();
        return arr;
    }

    fn update_certain_tile_internal(&mut self, y: i32, x: i32, value: i32) {
        self.game_state[y as usize][x as usize] = value;
    }

    fn random_coordinate(&mut self) -> i32 {
        self.randomiser.gen_range(0..3)
    }
}

#[test]
fn test_move_tiles_internal() {
    use crate::tiles_state::TilesState;
    let game_board_state = TilesState::new();
    assert_eq!(
        game_board_state.move_tiles_internal([2, 2, 8, 4]),
        [0, 4, 8, 4]
    );
    assert_eq!(
        game_board_state.move_tiles_internal([2, 2, 2, 2]),
        [0, 0, 4, 4]
    );
    assert_eq!(
        game_board_state.move_tiles_internal([16, 16, 16, 16]),
        [0, 0, 32, 32]
    );
}

#[test]
fn test_merge_matching_pair() {
    assert_eq!(TilesState::merge_matching_pair([2, 0, 0, 2]), [0, 0, 0, 4]);
    assert_eq!(TilesState::merge_matching_pair([2, 2, 0, 2]), [2, 0, 0, 4]);
    assert_eq!(TilesState::merge_matching_pair([2, 2, 2, 2]), [0, 4, 0, 4]);
    assert_eq!(TilesState::merge_matching_pair([2, 0, 4, 2]), [2, 0, 4, 2]);
}

#[test]
fn test_move_after_merge() {
    assert_eq!(TilesState::move_after_merge([2, 0, 0, 2]), [0, 0, 2, 2]);
    assert_eq!(TilesState::move_after_merge([4, 0, 0, 2]), [0, 0, 4, 2]);
    assert_eq!(TilesState::move_after_merge([2, 2, 0, 2]), [0, 2, 2, 2]);
    assert_eq!(TilesState::move_after_merge([2, 2, 0, 0]), [0, 0, 2, 2]);
    assert_eq!(TilesState::move_after_merge([0, 2, 0, 2]), [0, 0, 2, 2]);
    assert_eq!(TilesState::move_after_merge([4, 8, 0, 2]), [0, 4, 8, 2]);
}
