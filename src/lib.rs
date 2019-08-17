#[macro_use]
extern crate lazy_static;

mod osa;
mod key_layout;
pub use key_layout::{KeyLayout, Layouts};
pub use osa::{Metric, distance, generate};

// use std::rc::Rc;


// Fuctions with predefined costs

// lazy_static!{
//     static ref simple_metric: osa::Metric = osa::Metric {
//         deletion_cost: Box::new(|_, _, _| 1.0),
//         insertion_cost: Box::new(|_, _, _, _| 1.0),
//         substitution_cost: Box::new(|_, _, a, b| (if b == a { 0.0 } else { 1.0 })),
//         transposition_cost: Box::new(|_, _, a, b| (if b == a { 0.0 } else { 1.0 }))
//     };
// }

pub fn get_simple_metric() -> osa::Metric {
    osa::Metric {
        deletion_cost: Box::new(|_, _, _| 1.0),
        insertion_cost: Box::new(|_, _, _, _| 1.0),
        substitution_cost: Box::new(|_, _, a, b| (if b == a { 0.0 } else { 1.0 })),
        transposition_cost: Box::new(|_, _, a, b| (if b == a { 0.0 } else { 1.0 })),
        possible_chars: Layouts::QWERTZ.chars().collect(), 
    }
}

pub fn get_layout_metric(layout: &'static key_layout::KeyLayout) -> osa::Metric {
    let dist = move |a, b| layout_dist(layout, a, b);
    osa::Metric {
        deletion_cost: Box::new(|_, _, _| 1.0),
        insertion_cost: Box::new(move |s, i, insert, prev| insertion_dist(dist, s, i, insert, prev)),
        substitution_cost: Box::new(move |_, _, swap, cur| dist(swap, cur)),
        transposition_cost: Box::new(transposition_dist),
        possible_chars: layout.chars().collect(),
    }
}

pub fn get_layout_metric_mobile(layout: &'static key_layout::KeyLayout) -> osa::Metric {
    let dist = move |a, b| layout_dist_mobile(layout, a, b);
    osa::Metric {
        deletion_cost: Box::new(|_, _, _| 1.0),
        insertion_cost: Box::new(move |s, i, insert, prev| insertion_dist(dist, s, i, insert, prev)),
        substitution_cost: Box::new(move |_, _, swap, cur| dist(swap, cur)),
        transposition_cost: Box::new(transposition_dist),
        possible_chars: layout.chars().collect(),
    }
}

fn transposition_dist(_s: &str, _i: usize, cur: char, prev: char) -> f32 {
    if cur == prev { 0.0 } else { 0.6 }
}

fn insertion_dist(dist: impl Fn(char, char) -> f32, s: &str, i: usize, insert: char, prev: char) -> f32 {
    //TODO min ob layout_dist cur/prev and cur/next
    let next = s.char_indices().skip_while(|(j, _)| *j as i32 <= i as i32 - 1).next();
    let next_cost = next.map(|(_, x)| dist(insert, x) ).unwrap_or(1.0);
    let prev_cost = dist(insert, prev);
    osa::partial_min(&[next_cost, prev_cost]) + 0.4
}

fn layout_dist(layout: &key_layout::KeyLayout, a: char, b: char) -> f32 {
    if let (Some(pos_a), Some(pos_b)) = (layout.get_pos(a), layout.get_pos(b)) {
        let pos_a = (pos_a.0 as i32, pos_a.1 as i32, pos_a.2 as i32);
        let pos_b = (pos_b.0 as i32, pos_b.1 as i32, pos_b.2 as i32);
        let mut result = 0.0;
        if pos_a.2 < 2 && pos_b.2 < 2 {
            result = ((pos_a.0 - pos_b.0).pow(2) as f32 +
                      (pos_a.1 - pos_b.1).pow(2) as f32 +
                      ((pos_a.2 - pos_b.2) as f32 * 1.5).powi(2)).sqrt() / (2.5f32).sqrt();
        } else {
            result = (((pos_a.0 - pos_b.0).pow(2) + (pos_a.1 - pos_b.1).pow(2)) as f32).sqrt() / (2.0f32).sqrt();
            result += (pos_a.2 - pos_b.2).abs() as f32 * 2.0
        }
        result
    } else {
        1.0
    }
}

fn layout_dist_mobile(layout: &key_layout::KeyLayout, a: char, b: char) -> f32 {
    if let (Some(pos_a), Some(pos_b)) = (layout.get_pos(a), layout.get_pos(b)) {
        let pos_a = (pos_a.0 as i32, pos_a.1 as i32, pos_a.2 as i32);
        let pos_b = (pos_b.0 as i32, pos_b.1 as i32, pos_b.2 as i32);
        let mut result = 0.0;
        if pos_a.2 < 2 && pos_b.2 < 2 {
            result = ((pos_a.0 - pos_b.0).pow(2) as f32 +
                      (pos_a.1 - pos_b.1).pow(2) as f32 +
                      ((pos_a.2 - pos_b.2) as f32 * 1.5).powi(2)).sqrt() / (2.5f32).sqrt();
            // result += (pos_a.2 - pos_b.2).abs() as f32
        } else if pos_a.0 == pos_b.0 && pos_a.1 == pos_b.1 {
            result = ((pos_a.2 - pos_b.2) as f32).powi(2).sqrt() / (2f32).sqrt();
        } else {
            result = (((pos_a.0 - pos_b.0).pow(2) + (pos_a.1 - pos_b.1).pow(2)) as f32).sqrt() / (2.0f32).sqrt();
            result += (pos_a.2 - pos_b.2).abs() as f32 * 2.0
        }
        result
    } else {
        1.0
    }
}


/// deletions, insertions cost 1
/// substitutions, transpositions cost 1 if different chars 
pub fn osa_distance_simple(a: &str, b: &str) -> f32 {
    osa::distance(a, b, &get_simple_metric())
}

pub fn osa_distance_layout(a: &str, b: &str, layout: &'static key_layout::KeyLayout) -> f32 {
    osa::distance(a, b, &get_layout_metric(layout))
}

pub fn generate_simple(a: &str, max_cost: f32) -> Vec<osa::DistanceCost> {
    // let abc = ('a'..='z').chain('A'..='Z');
    // union with chars in `a`
    osa::generate(a, max_cost, &get_simple_metric())
}

pub fn generate_layout(a: &str, max_cost: f32, layout: &'static key_layout::KeyLayout) -> Vec<osa::DistanceCost> {
    osa::generate(a, max_cost, &get_layout_metric(&layout))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_distance() {
        assert!(osa_distance_simple("matthias", "matthias") == 0.0);
    }

    #[test]
    fn basic_distance() {
        assert!(osa_distance_simple("matthias", "mstthias") > 0.0);
        assert!(osa_distance_simple("matthias", "matthiass") > 0.0);
        assert!(osa_distance_simple("matthias", "mmatthias") > 0.0);
        assert!(osa_distance_simple("matthias", "amtthias") > 0.0);
        assert!(osa_distance_simple("matthias", "mathias") > 0.0);
    }

    #[test]
    fn miss_type_distance() {
        let a_typo = osa_distance_layout("matthias", "mstthias", &Layouts::QWERTZ);
        let a_typo_middle = osa_distance_layout("matthias", "metthias", &Layouts::QWERTZ);
        let a_typo_large = osa_distance_layout("matthias", "mntthias", &Layouts::QWERTZ);
        let i_repeat = osa_distance_layout("matthias", "matthiias", &Layouts::QWERTZ);
        let ia_swap = osa_distance_layout("matthias", "matthais", &Layouts::QWERTZ);
        println!("{} {} {}", a_typo, a_typo_middle, a_typo_large);
        assert!(a_typo < a_typo_middle);
        assert!(a_typo < a_typo_large);
        assert!(a_typo_middle < a_typo_large);
    }

    #[test]
    fn test_generate() {
        let typos = gen_typos_layout("matthias", 1.5, &Layouts::QWERTZ);
        // let typo_words: Vec<&str> = typos.iter().map(|dcost| dcost.word).cloned().collect();
        // assert!(typo_words.contains(&"amtthias"));
        // assert!(typo_words.contains("mathias"));
        // assert!(typo_words.contains("mathias"));
    }
}
