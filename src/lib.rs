pub fn modulo(a: usize, b: usize) -> usize {
    let (_, r) = hax_dep2::euc(a, b);
    r
}
