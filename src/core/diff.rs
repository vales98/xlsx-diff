use serde::Serialize;
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DiffItem<T> {
    pub op: String,
    pub old_index: Option<usize>,
    pub new_index: Option<usize>,
    pub value: T,
}
/**
 * Myers diff algorithm
 * @param a old data
 * @param b new data
 * @returns diff result
 * TODO: ignore some columns when using the 'header row' option
 */
pub fn myers_diff<T: Eq + Clone>(a: &[T], b: &[T]) -> Vec<DiffItem<T>> {
    let n = a.len();
    let m = b.len();
    let max = n + m;
    let mut v = vec![0; 2 * max + 2];
    let offset = max;
    let mut path = Vec::new();
    for d in 0..=max {
        let mut v_prev = v.clone();
        for k in (-(d as isize)..=d as isize).step_by(2) {
            let idx = (k + offset as isize) as usize;
            let (x_start, k_prev) = if k == -(d as isize) || (k != d as isize && v[(k -1 + offset as isize) as usize] < v[(k + 1 + offset as isize) as usize]) {
                (v[(k + 1 + offset as isize) as usize], k + 1)
            } else {
                (v[(k - 1 + offset as isize) as usize] + 1, k - 1)
            };
            let mut x = x_start;
            let mut y = x as isize - k;

            while x < n && y < m as isize && a[x] == b[y as usize] {
                x += 1;
                y += 1;
            }
            v[idx] = x;
            if x >= n && y >= m as isize {
                // Reconstruct the path
                let mut ops = Vec::new();
                let mut x = n;
                let mut y = m as isize;
                for d_back in (0..=d).rev() {
                    let k = x as isize - y;
                    let idx = (k + offset as isize) as usize;
                    let k_prev = if k == -(d_back as isize) as isize || (k != d_back as isize && v_prev[(k -1 + offset as isize) as usize] < v_prev[(k +1 + offset as isize) as usize]) {
                        k + 1
                    } else {
                        k - 1
                    };
                    let x_prev = v_prev[(k_prev + offset as isize) as usize];
                    let y_prev = x_prev as isize - k_prev;

                    while x > x_prev && y > y_prev {
                        x -= 1;
                        y -= 1;
                        // ops.push(DiffItem {
                        //     op: "Equal".to_string(),
                        //     old_index: Some(x),
                        //     new_index: Some(y as usize),
                        //     value: a[x].clone(),
                        // });
                    }
                    if x > x_prev && x > 0 {
                        x -= 1;
                        ops.push(DiffItem {
                            op: "Delete".to_string(),
                            old_index: Some(x),
                            new_index: None,
                            value: a[x].clone(),
                        });
                    } else if y > y_prev && y > 0 {
                        y -= 1;
                        ops.push(DiffItem {
                            op: "Insert".to_string(),
                            old_index: None,
                            new_index: Some(y as usize),
                            value: b[y as usize].clone(),
                        });
                    }
                    v_prev = path.pop().unwrap_or(v_prev);
                }
                ops.reverse();
                return ops;
            }
        }
        path.push(v.clone());
    }
    Vec::new()
}
