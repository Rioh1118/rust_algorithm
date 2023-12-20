pub struct QuickUnionUF{
    id: Vec<u32>,
}


impl QuickUnionUF {
    pub fn new(n: u32) -> QuickUnionUF {
        let mut id = Vec::new();
        for i in 0..n {
            id.push(i);
        }
        QuickUnionUF{ id }
    }

    fn root(&mut self, mut i: u32) -> u32 {
        while i != self.id[i as usize] {
            i = self.id[i as usize];
        }
        i
    }

    pub fn connected(&mut self, p: u32, q: u32) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self, p: u32, q: u32) {
        let i = self.root(p);
        let j = self.root(q);
        self.id[i as usize] = j;
    }
}