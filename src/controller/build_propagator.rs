use log::info;

use crate::model::pattern_data::Pattern;
use crate::model::pattern_data::PatternData;
use crate::model::pattern_propagator::PatternPixel;
use crate::model::pattern_propagator::PatternPropagator;
use crate::view::progress_bar::end_progress_bar;
use crate::view::progress_bar::print_progress_bar;
use crate::view::progress_bar::start_progress_bar;

pub fn build_propagator(pattern_data: PatternData) -> PatternPropagator {
    info!("building propagator...");
    let propagator = initialize_pixels(pattern_data);
    let propagator = calculate_propagator_relationships(propagator);
    info!("done!");

    propagator
}

fn initialize_pixels(pattern_data: PatternData) -> PatternPropagator {
    let mut pattern_pixels = Vec::new();
    let PatternData {
        ref patterns,
        image_height,
        image_width,
        pattern_width,
        pattern_height,
    } = pattern_data;
    let pattern_size = pattern_width * pattern_height;
    let total_weight = pattern_size * image_width * image_height;

    for i in 0..patterns.len() {
        println!("{:?}", patterns[i].pixels);
        for y in 0..pattern_height {
            for x in 0..pattern_width {
                let j = (pattern_width * y + x) as usize;
                //println!("{:?}", pattern_data.get_pixel_index(i, x, y));
                pattern_pixels.push(PatternPixel {
                    color: patterns[i].pixels[j],
                    colors: patterns[i].pixels.clone(),
                    relationships: Vec::new(),
                    weight: patterns[i].weight,
                    x,
                    y,
                });
            }
        }
    }

    let propagator = PatternPropagator {
        pattern_data,
        pattern_pixels,
        total_weight,
    };

    propagator
}

fn calculate_propagator_relationships(
    mut pattern_propagator: PatternPropagator,
) -> PatternPropagator {
    let pattern_data = &pattern_propagator.pattern_data;
    let PatternData {
        ref patterns,
        pattern_width,
        pattern_height,
        ..
    } = *pattern_data;

    start_progress_bar();
    for this_pattern_index in 0..patterns.len() {
        let this_colors = &patterns[this_pattern_index].pixels;
        for y1 in 0..pattern_height {
            for x1 in 0..pattern_width {
                let relationships =
                    calculate_pixel_relationships(pattern_data, this_colors, x1, y1);

                let pi = pattern_data.get_pixel_index(this_pattern_index, x1, y1);

                pattern_propagator.pattern_pixels[pi].relationships = relationships;
            }
        }

        print_progress_bar(100 * (this_pattern_index + 1) / patterns.len());
    }
    end_progress_bar();

    pattern_propagator
}

fn calculate_pixel_relationships(
    pattern_data: &PatternData,
    this_colors: &Vec<u32>,
    x1: u32,
    y1: u32,
) -> Vec<bool> {
    let PatternData {
        ref patterns,
        pattern_width,
        pattern_height,
        ..
    } = *pattern_data;

    let w = pattern_width;
    let h = pattern_height;
    let s = (pattern_width * pattern_height) as usize;
    let total_relationships = patterns.len() * s * s;
    let mut relationships = vec![false; total_relationships];

    for that_pattern_index in 0..patterns.len() {
        let that_colors = &patterns[that_pattern_index].pixels;
        for y2 in 0..h {
            for x2 in 0..w {
                for v in 0..h {
                    for u in 0..w {
                        let tx = u as i32 - x2 as i32;
                        let ty = v as i32 - y2 as i32;

                        let ri =
                            pattern_data.get_relationship_index(that_pattern_index, x2, y2, u, v);

                        let inside_x_interval = is_inside_interval_intersection(x1, tx, w);
                        let inside_y_interval = is_inside_interval_intersection(y1, ty, h);

                        if !inside_x_interval || !inside_y_interval {
                            relationships[ri] = false;
                            continue;
                        }

                        // TODO: speed up with caching or clever index traversion
                        let intersection_match =
                            is_intersection_match(this_colors, that_colors, tx, ty, w, h);

                        relationships[ri] = intersection_match;
                    }
                }
            }
        }
    }

    relationships
}

// TODO: proper rust doc
// Example:
// this interval:      | |x| | | |
// other interval: | | | | | |
// x = 1
// other_interval_start = -2
// width = 5
// Result: true, as x lies inside the 3 unit wide intersection of both intervals
fn is_inside_interval_intersection(x: u32, other_interval_start: i32, width: u32) -> bool {
    if other_interval_start < 0 {
        return (x as i32) < other_interval_start + width as i32;
    } else if other_interval_start > 0 {
        return (x as i32) >= other_interval_start;
    }
    true
}

fn is_intersection_match(
    this_colors: &Vec<u32>,
    that_colors: &Vec<u32>,
    that_pattern_x: i32,
    that_pattern_y: i32,
    pattern_width: u32,
    pattern_height: u32,
) -> bool {
    let x = that_pattern_x;
    let y = that_pattern_y;
    let w = pattern_width as i32;
    let h = pattern_height as i32;

    let overlap_width = if x <= 0 { w + x } else { w - x };
    let overlap_height = if y <= 0 { h + y } else { h - y };
    let this_left = if x <= 0 { 0 } else { x };
    let this_top = if y <= 0 { 0 } else { y };
    let that_left = if x <= 0 { -x } else { 0 };
    let that_top = if y <= 0 { -y } else { 0 };

    for v in 0..overlap_height {
        for u in 0..overlap_width {
            let this_u = this_left + u;
            let this_v = this_top + v;
            let that_u = that_left + u;
            let that_v = that_top + v;
            let this_index = (w * this_v + this_u) as usize;
            let that_index = (w * that_v + that_u) as usize;

            if this_colors[this_index] != that_colors[that_index] {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::extract_patterns::extract_patterns;
    use crate::controller::load_image::load_image;

    #[test]
    fn it_initializes_pattern_pixels() {
        let image = load_image("./data/flowers.png");
        let pattern_data = extract_patterns(image, 3, 3);
        let pattern_propagator = initialize_pixels(pattern_data);
        let pattern_pixels = pattern_propagator.pattern_pixels;

        let mut total_weight = 0;
        for i in 0..pattern_pixels.len() {
            total_weight += pattern_pixels[i].weight;
        }

        assert_eq!(pattern_propagator.total_weight, total_weight);
    }

    #[test]
    fn it_checks_if_pattern_intersections_match() {
        #[rustfmt::skip]
        let c1 = vec![
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 2, 2,
        ];
        #[rustfmt::skip]
        let c2 = vec![
            2, 2, 1, 1, 1,
            1, 1, 3, 1, 1,
            1, 1, 1, 1, 1,
        ];
        let w = 5;
        let h = 3;

        assert!(!is_intersection_match(&c1, &c2, 0, 0, w, h));
        assert!(!is_intersection_match(&c1, &c2, -2, 0, w, h));
        assert!(is_intersection_match(&c1, &c2, 10, 50, w, h));
        assert!(is_intersection_match(&c1, &c2, -20, 0, w, h));
        assert!(is_intersection_match(&c1, &c2, -3, 0, w, h));
        assert!(is_intersection_match(&c1, &c2, -3, -1, w, h));
        assert!(is_intersection_match(&c1, &c2, 3, 2, w, h));
    }

    mod integration {
        use crate::model::image::Image;

        use super::*;

        #[test]
        fn it_builds_a_propagator_from_a_simple_example() {
            let pattern_width = 3;
            let pattern_height = 2;
            let image = Image {
                width: 4,
                height: 3,
                #[rustfmt::skip]
                data: vec![
                    0, 0, 0, 0,
                    0, 1, 1, 1,
                    0, 1, 2, 0,
                ],
            };

            let pattern_data = extract_patterns(image, pattern_width, pattern_height);

            // make sure there are no deduplcations to make calculating the pattern index easier
            assert_eq!(
                pattern_data.patterns.len(),
                (pattern_data.image_width * pattern_data.image_height) as usize,
            );

            let propagator = build_propagator(pattern_data);

            let pi = propagator.pattern_data.get_pixel_index(5, 0, 1);
            println!("{:?}", propagator.pattern_pixels[pi].colors);
            //assert_eq!(propagator.pattern_pixels[pi].color, 1)
        }
    }
}
