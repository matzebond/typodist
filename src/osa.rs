use std::cmp::Ordering::{self, Equal};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::iter;

const cost_multiplier: f32 = 1.4;

pub struct Metric {
    // delete at postiton, cur
    pub deletion_cost: Box<dyn Fn(&str, usize, char) -> f32>,
    // insert char at before postiton, prev
    pub insertion_cost: Box<dyn Fn(&str, usize, char, char) -> f32>,
    // substitute at position with char, there was
    pub substitution_cost: Box<dyn Fn(&str, usize, char, char) -> f32>,
    // transpose at position with in front, cur + prev
    pub transposition_cost: Box<dyn Fn(&str, usize, char, char) -> f32>,
    // possible chars (only important for generation)
    pub possible_chars: Vec<char>,
}

pub fn distance(a: &str, b: &str, metric: &Metric) -> f32 {
    let mut d: Vec<Vec<f32>> = vec![vec![0.0; b.chars().count()+1]; a.chars().count()+1];
    
    for i in 0..=a.len() {
        d[i][0] = i as f32;
    }

    for j in 0..=b.len() {
        d[0][j] = j as f32;
    }
    
    let mut a_prev_char = '\0';
    let mut b_prev_char = '\0';
    for ((a_index, a_char), i) in a.char_indices().zip(1..) {
        for ((b_index, b_char), j) in b.char_indices().zip(1..) {
            let del_cost = d[i-1][j] + (metric.deletion_cost)(b, b_index, b_char);
            let ins_cost = d[i][j-1] + (metric.insertion_cost)(b, b_index, a_char, b_char);
            let sub_cost = d[i-1][j-1] + (metric.substitution_cost)(b, b_index, a_char, b_char);


            if i > 1 && j > 1 && a_char == b_prev_char && a_prev_char == b_char {
                let trans_cost = d[i-2][j-2] + (metric.transposition_cost)(b, b_index, b_char, b_prev_char);
                d[i][j] = *partial_min(&[del_cost, ins_cost, sub_cost, trans_cost]);
            } else {
                d[i][j] = *partial_min(&[del_cost, ins_cost, sub_cost]);
            }
            b_prev_char = b_char;
        }
        a_prev_char = a_char;
    }

    d[a.len()][b.len()]
}

pub fn partial_min<T: PartialOrd + Copy>(_varags: &[T]) -> &T{
    _varags.into_iter().fold(&_varags[0], |acc, x| (if x <= acc { x } else { acc }))
}

pub fn partial_max<T: PartialOrd + Copy>(_varags: &[T]) -> T{
    _varags.into_iter().fold(_varags[0], |acc, &x| (if x > acc { x } else { acc }))
}

#[derive(PartialEq, Debug)]
pub struct DistanceCost {
    pub cost: f32,
    pub word: String,
}

impl PartialOrd for DistanceCost {
    fn partial_cmp(&self, other: &DistanceCost) -> Option<Ordering> {
        // ordering fliped
        other.cost.partial_cmp(&self.cost)
    }
}

//ignore f32 problems
impl Eq for DistanceCost {}
impl Ord for DistanceCost {
    fn cmp(&self, other: &DistanceCost) -> Ordering {
        self.partial_cmp(other).unwrap_or(Equal)
    }
}

pub fn generate(a: &str, max_cost: f32, metric: &Metric) -> Vec<DistanceCost> {
    let mut heap = BinaryHeap::new();
    heap.push(DistanceCost{ cost: 0.0, word: a.to_string()});
    let mut processed = HashSet::new();
    processed.insert(a.to_string());
    let mut results = Vec::new();

    while let Some(cur) = heap.pop() {
        if cur.cost > max_cost {
            break
        }

        // split in two &str at every postion
        let splits = cur.word.char_indices().map(|(i, _)| {
            let split = cur.word.split_at(i);
            (i, split.0, split.1)
        }).chain(iter::once((cur.word.len(), &cur.word[..], "")));

        for ((i, front, back), num) in splits.zip(0..) {
            // println!("{}: {} - {}", i, front, back);

            if num > 0 {
                //deletion
                let mut word: String = front.to_string();
                let cur_char = word.pop().unwrap();
                word.push_str(&back);
                let cost = cur.cost * cost_multiplier + (metric.deletion_cost)(&cur.word, i, cur_char);
                if !processed.contains(&word) && cost <= max_cost {
                    heap.push(DistanceCost { word: word.clone(), cost });
                    processed.insert(word);
                }
            }

            if num > 1 {
                //transposition
                let mut word = front.to_string();
                let cur_char = word.pop().unwrap();
                let prev_char = word.pop().unwrap();
                word.push(cur_char);
                word.push(prev_char);
                word.push_str(&back);
                let cost = cur.cost * cost_multiplier + (metric.transposition_cost)(&cur.word, i, cur_char, prev_char);
                if !processed.contains(&word) && cost <= max_cost {
                    heap.push(DistanceCost { word: word.clone(), cost });
                    processed.insert(word);
                }
            }


            for &c in &metric.possible_chars {
                //insertions
                let mut word: String = front.to_string();
                let prev_char = word.pop();
                if let Some(prev_char) = prev_char {
                    word.push(prev_char);
                }
                word.push(c);
                word.push_str(&back);
                let cost = cur.cost * cost_multiplier + (metric.insertion_cost)(&cur.word, i, c, prev_char.unwrap_or('\0'));
                if !processed.contains(&word) && cost <= max_cost {
                    heap.push(DistanceCost { word: word.clone(), cost });
                    processed.insert(word);
                }

                //substitution
                if num > 0 {
                    let mut word: String = front.to_string();
                    let cur_char = word.pop().unwrap();
                    word.push(c);
                    word.push_str(&back);
                    let cost = cur.cost * cost_multiplier + (metric.substitution_cost)(&cur.word, i, c, cur_char);
                    if !processed.contains(&word) && cost <= max_cost {
                        heap.push(DistanceCost { word: word.clone(), cost });
                        processed.insert(word);
                    }
                }
            }
        }

        // no changes
        results.push(cur);
    }
    results
}

// fn get_after_deletion(a: &str, i: usize) -> String {
//     a.to_string()
// }


