#[cfg(test)]
mod tests {
    use rust_algorithm::quick_union_uf::QuickUnionUF;

    #[test]
    fn test_quick_union_uf() {
        let mut qu = QuickUnionUF::new(10);

        // Initially, no elements are connected.
        for i in 0..10 {
            for j in 0..10 {
                if i != j {
                    assert!(!qu.connected(i, j));
                }
            }
        }

        // Connect some elements and test.
        qu.union(1, 2);
        assert!(qu.connected(1, 2));

        qu.union(3, 4);
        assert!(qu.connected(3, 4));
        assert!(!qu.connected(1, 3));

        // Connect more elements and test.
        qu.union(2, 3);
        assert!(qu.connected(1, 3));
        assert!(qu.connected(2, 4));
    }
}