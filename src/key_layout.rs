use std::collections::HashMap;

pub mod Layouts {
use super::*;
lazy_static! {
pub static ref QWERTZ: KeyLayout = KeyLayout::new(
    vec![
        vec![vec!['^'],vec!['1','!'],vec!['2','"'],vec!['3', '§'],vec!['4','$'],vec!['5','%'],vec!['6','&'],vec!['7','/'],vec!['8','('],vec!['9',')'],vec!['0','='],vec!['ß','?'],vec!['`']],
        vec![vec![],vec!['q','Q','@'],vec!['w','W'],vec!['e','E','€'],vec!['r','R'],vec!['t','T'],vec!['z','Z'],vec!['u','U'],vec!['i','I'],vec!['o','O'],vec!['p','P'],vec!['ü','Ü'],vec!['+','*','~']],
        vec![vec![],vec!['a','A'],vec!['s','S'],vec!['d','D'],vec!['f','F'],vec!['g','G'],vec!['h','H'],vec!['j','J'],vec!['k','K'],vec!['l','L'],vec!['ö','Ö'],vec!['ä','Ä'],vec!['#','\'']],
        vec![vec!['<','>','|'],vec!['y','Y'],vec!['x','X'],vec!['c','C'],vec!['v','V'],vec!['b','B'],vec!['n','N'],vec!['m','M'],vec![',',';'],vec!['.',':'],vec!['-','_']],
    ],
);

pub static ref QWERTZ_ANYSOFT_EXTRA: KeyLayout = KeyLayout::new(
    vec![
        //ext_kbd_top_row_numbers_simple
        vec![vec!['1', '!'],vec!['2', '@'],vec!['3','#'],vec!['4','$',';'],vec!['5','%',':'],vec!['6','^'],vec!['7','&'],vec!['8','*'],vec!['9','(','[','{','/'],vec!['0',')',']','}','-','=','_','+','°']],
        vec![vec!['q','Q','@'],vec!['w','W'],vec!['e','E','€'],vec!['r','R'],vec!['t','T'],vec!['z','Z'],vec!['u','U', 'ü'],vec!['i','I'],vec!['o','O','ö'],vec!['p','P']],
        vec![vec!['a','A'],vec!['s','S','ß'],vec!['d','D'],vec!['f','F'],vec!['g','G'],vec!['h','H'],vec!['j','J'],vec!['k','K'],vec!['l','L']],
        //ext_kbd_bottom_row_regular
        vec![vec![',',':',';','-','\''],vec!['y','Y'],vec!['x','X'],vec!['c','C'],vec!['v','V'],vec!['b','B'],vec!['n','N'],vec!['m','M'],vec!['.','!','?']],
    ],
);
}
}

pub type KeyIndex = (usize, usize, usize);

pub struct KeyLayout {
    matrix: Vec<Vec<Vec<char>>>,
    char_indicies: HashMap<char, (usize,usize,usize)>,
}

impl KeyLayout {
    pub fn new(matrix: Vec<Vec<Vec<char>>>) -> KeyLayout {
        let char_indicies = KeyLayout::chars_indexed(&matrix).into_iter().collect();
        KeyLayout { matrix, char_indicies }
    }

    pub fn chars_indexed(matrix: &Vec<Vec<Vec<char>>>) -> Vec<(char, KeyIndex)> {
        let mut result: Vec<(char,(usize,usize,usize))> = Vec::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                for k in 0..matrix[i][j].len() {
                    result.push((matrix[i][j][k], (i,j,k)));

                }
            }
        }
        result
    }

    // fn chars_indexed(&self) -> impl Iterator<Item = (char,(usize,usize,usize))> {
        // (0..).zip((0..).zip((0..).zip(self.matrix.iter()).flatten()).flatten()).map(|(n1,(n2,(n3,c)))| (c,(n1,n2,n3)));

        // self.matrix.iter().enumerate().map(|(n1,r)| (r.iter().enumerate().map(|(n2,k)| (k.iter().enumerate(),n2)),n1))

        // self.matrix.iter().zip(0..).map(|(r,n1)| (r.iter().zip(0..).map(|(k,n2)| (k.iter().zip(0..).map(|(c,n3)| ((c,n3),n2)),n1)))).flatten()
        //                                 .map(|(((c,n3),n2),n1)| (c,(n1,n2,n3)))

        // self.matrix.iter().map(|mut r| r.iter().map(|mut k| k.iter().enumerate()).enumerate()).enumerate()
    // }

    pub fn get_pos(&self, c: char) -> Option<KeyIndex> {
        self.char_indicies.get(&c).map(|&i| i)
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.char_indicies.keys().map(|&c| c)
    }

    pub fn dist(&self, a: char, b: char) -> Option<f32> {
        if let (Some(a_pos), Some(b_pos)) = (self.get_pos(a), self.get_pos(b)) {
            let dist = (((a_pos.0 as i32 - b_pos.0 as i32).pow(2)
                         + (a_pos.1 as i32 - b_pos.1 as i32).pow(2)
                         + (a_pos.2 as i32 - b_pos.2 as i32).pow(2)) as f32).sqrt();
            Some(dist)
        }
        else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qwertz_pos() {
        assert_eq!(QWERTZ.get_pos('^'), Some((0,0,0)));
    }

    #[test]
    fn qwertz_dist() {
        assert!(QWERTZ.dist('t', 'z').unwrap() == 1.0);
        assert!(QWERTZ.dist('t', 'g').unwrap() == 1.0);
        assert!(QWERTZ.dist('t', 'h').unwrap() > 1.0);
        assert!(QWERTZ.dist('t', 'h').unwrap() < 2.0);
        assert!(QWERTZ.dist('w', 'm').unwrap() > 5.0);
        assert!(QWERTZ.dist('w', 'm').unwrap() < 7.0);
    }
}
