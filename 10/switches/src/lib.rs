use std::cmp;

pub fn min_moves(target: u16, target_bits: usize, actions: &[u16]) -> Option<u32> {
    // we want to implement DP here
    // d[pattern][count] = minimum amount of actions needed to reach pattern using first count
    // actions
    //
    // initially d[pattern][0] = None, and d[0][0] = 0 - can reach the .... pattern for free
    // recursively, d[pattern][count] can lead to d[pattern][count + 1] (don't use action) and to
    // d[pattern ^ action][count + 1] (use action)
    // number of actions has to be min'ed with what's already in that cell
    // we can calculate the values forward, as it seems a bit simpler to follow
    //
    // we don't have to store all of the state, just prev and next
    //
    // even better, we can initialise next to same values as in prev and then we only need to
    // consider the branch when we're taking an action, as we already know what will happen if we
    // don't take it
    //
    // so at the end of each iteration we can just copy from next to prev and done!

    let max_target = 1 << target_bits;

    let mut prev: Vec<Option<u32>> = vec![None; max_target];
    let mut next: Vec<Option<u32>> = vec![None; max_target];

    prev[0] = Some(0);
    next[0] = Some(0);

    for action in actions {
        for (k, x) in prev.iter().enumerate() {
            if let &Some(x) = x {
                let k_with_action = (k as u16) ^ action;
                eprintln!("{} {} {} {:?} {:?}", action, k, x, k_with_action, next[k_with_action as usize]);
                next[k_with_action as usize] = next[k_with_action as usize]
                    .map(|y| cmp::min(x + 1, y))
                    .or(Some(x + 1));
            }
        }
        prev.copy_from_slice(next.as_slice());
    }

    prev[target as usize]
}

#[test]
fn single_action() {
    let target = 1;
    let target_bits = 1;
    let actions = &[1];

    let result = crate::min_moves(target, target_bits, actions);

    assert_eq!(result, Some(1));
}

#[test]
fn simple() {
    let target = 6;
    let target_bits = 4;
    let actions = &[8, 10, 4, 12, 5, 3];

    let result = crate::min_moves(target, target_bits, actions);

    // 5 ^ 3 = 6
    assert_eq!(result, Some(2));
}
