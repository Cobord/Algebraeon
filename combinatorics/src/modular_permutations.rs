use algebraeon_groups::examples::symmetric::Permutation;
use algebraeon_rings::finite_fields::modulo::Modulo;

pub fn modular_permutation<const N: usize>(
    f: impl Fn(Modulo<N>) -> Modulo<N>,
) -> Result<Permutation<N>, &'static str> {
    let mut perm = [0; N];
    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        perm[i] = f(i.into()).into();
    }
    Permutation::new(perm)
}

#[cfg(test)]
mod tests {
    use algebraeon_rings::structure::IntoErgonomic;

    use super::*;

    #[test]
    pub fn test() {
        println!(
            "{:?}",
            modular_permutation::<23>(|i| (i.into_ergonomic() * 7).into_verbose())
                .unwrap()
                .disjoint_cycles()
        );
    }
}
