use crate::model::pattern_propagator::PatternPropagator;
use crate::model::wave::Wave;

pub fn propagate(wave: &mut Wave, propagator: &PatternPropagator) {
    // TODO: implications for relationship array???
    //let mut index_stack = vec![(wave.last_index_collapsed, wave.last_index_collapsed)];
    let mut index_stack = Vec::new();
    let width = wave.width as usize;
    let height = wave.height as usize;

    put_wave_neighbors_on_stack(wave.last_index_collapsed, width, height, &mut index_stack);

    while let Some((i, j)) = index_stack.pop() {
        let ix = (i % width) as i32;
        let iy = (i / width) as i32;
        let jx = (j % width) as i32;
        let jy = (j / width) as i32;
        let relx = jx - ix;
        let rely = jy - iy;

        if wave.indices[j].len() == 0 {
            for ii in 0..wave.indices.len() {
                //println!("{} {}", ii, wave.indices[ii].len());
            }

            for y in 0..10 {
                for x in 0..10 {
                    let ii = 10 * y + x;
                    let len = wave.indices[ii].len();

                    if len < 1000 {
                        print!("{:3} ", len);
                    } else {
                        print!("xxx ");
                    }
                }
                println!();
            }

            todo!("implement contradiction handling");
        }

        let mut remove_occured = false;

        //println!(
        //    "popped: (({}, {}), ({}, {})), remaining: {:?}",
        //    ix, iy, jx, jy, index_stack
        //);

        let mut keeps = vec![false; wave.indices[j].len()];

        for k in 0..wave.indices[i].len() {
            let u = wave.indices[i][k];
            for l in 0..wave.indices[j].len() {
                let v = wave.indices[j][l];
                if let Some(r) = calculate_relationship_index(propagator, relx, rely, u, v) {
                    print!("r");
                    if propagator.pattern_pixels[u].relationships[r] {
                        keeps[l] = true;
                        print!("k");
                    }
                }
            }
        }

        for k in 0..keeps.len() {
            let l = keeps.len() - 1 - k;
            if !keeps[l] {
                wave.indices[j].swap_remove(l);
                remove_occured = true;
                //print!("{}|", wave.indices[j].len());
            }
        }
        println!();

        if remove_occured {
            put_wave_neighbors_on_stack(j, width, height, &mut index_stack);
        }
    }
}

/// Calculates the index inside the relationships vector of a pattern_pixel.
/// `relx` and `rely` are the relative coordinates inside the wave of the
/// `second_pixel_index` as seen from the `first_pixel_index`.
/// E.g. the first pixel is at (5, 5), the second at (7, 4), then
/// `relx = 2`, `rely = -1`
fn calculate_relationship_index(
    propagator: &PatternPropagator,
    relx: i32,
    rely: i32,
    first_pixel_index: usize,
    second_pixel_index: usize,
) -> Option<usize> {
    let first_pixel = &propagator.pattern_pixels[first_pixel_index];
    let x = first_pixel.x as i32 + relx;
    let y = first_pixel.y as i32 + rely;
    let w = propagator.pattern_data.pattern_width as i32;
    let h = propagator.pattern_data.pattern_height as i32;

    // TODO: build_propagator is wrong
    print!("{:?}", (x, y, relx, rely, first_pixel_index, second_pixel_index));

    if x < 0 || x >= w || y < 0 || y >= h {
        return None;
    }

    let s1 = w as usize;
    let s2 = (w * h) as usize;
    Some(second_pixel_index * s2 + y as usize * s1 + x as usize)
}

fn put_wave_neighbors_on_stack(
    index: usize,
    width: usize,
    height: usize,
    index_stack: &mut Vec<(usize, usize)>,
) {
    if index >= width * height {
        panic!("index overflow");
    }

    let row = index / width;
    let col = index % width;

    let has_top_row = row > 0;
    let has_bottom_row = row < height - 1;
    let has_left_col = col > 0;
    let has_right_col = col < width - 1;

    // Top neighbor
    if has_top_row {
        index_stack.push((index, index - width));

        // Top-left neighbor
        if has_left_col {
            index_stack.push((index, index - width - 1));
        }

        // Top-right neighbor
        if has_right_col {
            index_stack.push((index, index - width + 1));
        }
    }

    // Bottom neighbor
    if has_bottom_row {
        index_stack.push((index, index + width));

        // Bottom-left neighbor
        if has_left_col {
            index_stack.push((index, index + width - 1));
        }

        // Bottom-right neighbor
        if has_right_col {
            index_stack.push((index, index + width + 1));
        }
    }

    // Left neighbor
    if has_left_col {
        index_stack.push((index, index - 1));
    }

    // Right neighbor
    if has_right_col {
        index_stack.push((index, index + 1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_yyy() {
        let mut index_stack = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut i = 0;
        while i < index_stack.len() {
            if index_stack[i] == 0 || index_stack[i] == 9 {
                index_stack.swap_remove(i);
            } else {
                i += 1;
            }
        }

        println!("{:?}", index_stack);
    }

    #[test]
    fn it_xxx() {
        let mut index_stack = vec![(1, 1), (2, 1), (3, 1), (1, 4), (1, 5), (1, 6), (1, 8)];

        while let Some((i, j)) = index_stack.pop() {
            println!("{} {}", i, j);

            if i == 1 {
                index_stack.push((2, j));
            }
        }
    }
}
