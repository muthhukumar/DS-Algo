pub fn two_crystal_ball(list: &Vec<bool>) -> i32 {
    let jump = (list.len() as f32).sqrt() as usize;

    let mut end = 0;
    let range = 0..=list.len();

    for i in range.step_by(jump) {
        if list[i] {
            end = i;

            break;
        }
    }

    if end == 0 {
        return -1;
    }

    let range = end - jump;

    for j in range..list.len() {
        if list[j] {
            return j as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::two_crystal_ball;

    #[test]
    fn test_two_crystal_ball() {
        let input = vec![
            false, false, false, false, false, false, false, false, true, true, true,
        ];

        assert_eq!(8, two_crystal_ball(&input))
    }

    #[test]
    fn test_two_crystal_ball_fail() {
        let input = vec![
            false, false, false, false, false, false, false, false, false, false, false,
        ];

        assert_eq!(-1, two_crystal_ball(&input))
    }
}
