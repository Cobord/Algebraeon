use crate::{polynomial::*, structure::*};
use algebraeon_nzq::{traits::Fraction, *};
use algebraeon_sets::structure::MetaType;

pub fn root_sum_poly(p: &Polynomial<Integer>, q: &Polynomial<Integer>) -> Polynomial<Integer> {
    let x = Variable::new(String::from("x"));
    let z = Variable::new(String::from("z"));

    let p = p.apply_map(|c| MultiPolynomial::constant(c.clone()));
    let q = q.apply_map(|c| MultiPolynomial::constant(c.clone()));
    let r = Integer::structure()
        .multivariable_polynomial_ring()
        .polynomial_ring()
        .evaluate(
            &q,
            &MultiPolynomial::add(
                &MultiPolynomial::var(z.clone()),
                &MultiPolynomial::neg(&MultiPolynomial::var(x.clone())),
            ),
        )
        .expand(&x);

    let root_sum_poly = Integer::structure()
        .multivariable_polynomial_ring()
        .polynomial_ring()
        .resultant(p.clone(), r.clone())
        .expand(&z)
        .apply_map(|c| MultiPolynomial::as_constant(c).unwrap());
    root_sum_poly.primitive_squarefree_part()
}

pub fn root_product_poly(p: &Polynomial<Integer>, q: &Polynomial<Integer>) -> Polynomial<Integer> {
    let x = Variable::new(String::from("x"));
    let t = Variable::new(String::from("t"));

    let p = p.apply_map(|c| MultiPolynomial::constant(c.clone()));
    let q = Integer::structure()
        .multivariable_polynomial_ring()
        .polynomial_ring()
        .evaluate(
            &q.apply_map(|c| MultiPolynomial::constant(c.clone())),
            &MultiPolynomial::var(x.clone()),
        );
    let r = q.homogenize(&t).expand(&t);
    //x ** q.degree() * q(t * x ** -1)

    let root_prod_poly = Integer::structure()
        .multivariable_polynomial_ring()
        .polynomial_ring()
        .resultant(p.clone(), r.clone())
        .expand(&x)
        .apply_map(|c| MultiPolynomial::as_constant(c).unwrap());
    root_prod_poly.primitive_squarefree_part()
}

pub fn root_rat_mul_poly(poly: Polynomial<Integer>, rat: &Rational) -> Polynomial<Integer> {
    debug_assert!(rat != &Rational::ZERO);
    debug_assert!(poly.is_irreducible());
    //we are multiplying by a so need to replace f(x) with f(x/a)
    //e.g. f(x) = x-1 and multiply root by 3 then replace f(x) with
    //f(x/3) = 3/x-1 = x-3
    //e.g. f(x) = 1 + x + x^2 replace it with f(d/n * x) = 1 + d/n x + d^2/n^2 x^2 = n^2 + ndx + d^2 x
    let rat_mul_poly = Polynomial::from_coeffs({
        let degree = poly.degree().unwrap();
        let (n, d) = rat.numerator_and_denominator();
        let d = Integer::from(d);
        let mut n_pows = vec![Integer::from(1)];
        let mut d_pows = vec![Integer::from(1)];
        {
            let mut n_pow = n.clone();
            let mut d_pow = d.clone();
            for _i in 0..degree {
                n_pows.push(n_pow.clone());
                d_pows.push(d_pow.clone());
                n_pow *= &n;
                d_pow *= &d;
            }
        }
        debug_assert_eq!(n_pows.len(), degree + 1);
        debug_assert_eq!(d_pows.len(), degree + 1);
        let coeffs = poly
            .into_coeffs()
            .iter()
            .enumerate()
            .map(|(i, c)| &d_pows[i] * &n_pows[degree - i] * c)
            .collect();
        coeffs
    })
    .primitive_part()
    .unwrap();
    debug_assert!(rat_mul_poly.is_irreducible());
    rat_mul_poly
}

pub fn evaluate_at_rational(poly: &Polynomial<Integer>, val: &Rational) -> Rational {
    poly.apply_map(|x| Rational::from(x)).evaluate(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_sum_poly() {
        for (f, g, exp) in vec![
            (
                Polynomial::from_coeffs(vec![Integer::from(0)]),
                Polynomial::from_coeffs(vec![Integer::from(0)]),
                Polynomial::from_coeffs(vec![Integer::from(0)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(0)]),
                Polynomial::from_coeffs(vec![Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(0)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(1)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(-3), Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(-5), Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(-8), Integer::from(1)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(-7), Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(1)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(-1), Integer::from(2)]),
                Polynomial::from_coeffs(vec![Integer::from(-1), Integer::from(3)]),
                Polynomial::from_coeffs(vec![Integer::from(-5), Integer::from(6)]),
            ),
            (
                Polynomial::from_coeffs(vec![
                    Integer::from(-1),
                    Integer::from(-2),
                    Integer::from(1),
                ]),
                Polynomial::from_coeffs(vec![
                    Integer::from(-2),
                    Integer::from(0),
                    Integer::from(1),
                ]),
                Polynomial::from_coeffs(vec![
                    Integer::from(-7),
                    Integer::from(5),
                    Integer::from(3),
                    Integer::from(-1),
                ]),
            ),
        ] {
            println!();
            let rsp = root_sum_poly(&f, &g);
            println!("f = {}", Polynomial::to_string(&f));
            println!("g = {}", Polynomial::to_string(&g));
            println!(
                "exp = {}    exp_factored = {:?}",
                Polynomial::to_string(&exp),
                Integer::structure()
                    .polynomial_ring()
                    .factorize_by_kroneckers_method(exp.clone(), Integer::factor)
            );
            println!(
                "rsp = {}    rsp_factored = {:?}",
                Polynomial::to_string(&rsp),
                Integer::structure()
                    .polynomial_ring()
                    .factorize_by_kroneckers_method(rsp.clone(), Integer::factor)
            );
            assert!(Polynomial::are_associate(&exp, &rsp));
        }
    }

    #[test]
    fn test_root_prod_poly() {
        for (f, g, exp) in vec![
            (
                Polynomial::from_coeffs(vec![Integer::from(0)]),
                Polynomial::from_coeffs(vec![Integer::from(0)]),
                Polynomial::from_coeffs(vec![Integer::from(0)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(0)]),
                Polynomial::from_coeffs(vec![Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(0)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(1)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(-3), Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(-5), Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(-15), Integer::from(1)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(-7), Integer::from(1)]),
                Polynomial::from_coeffs(vec![Integer::from(1)]),
            ),
            (
                Polynomial::from_coeffs(vec![Integer::from(-1), Integer::from(2)]),
                Polynomial::from_coeffs(vec![Integer::from(-1), Integer::from(3)]),
                Polynomial::from_coeffs(vec![Integer::from(-1), Integer::from(6)]),
            ),
            (
                Polynomial::from_coeffs(vec![
                    Integer::from(-1),
                    Integer::from(-2),
                    Integer::from(1),
                ]),
                Polynomial::from_coeffs(vec![
                    Integer::from(-2),
                    Integer::from(0),
                    Integer::from(1),
                ]),
                Polynomial::from_coeffs(vec![
                    Integer::from(4),
                    Integer::from(0),
                    Integer::from(-12),
                    Integer::from(0),
                    Integer::from(1),
                ]),
            ),
            (
                Polynomial::from_coeffs(vec![
                    Integer::from(-2),
                    Integer::from(0),
                    Integer::from(1),
                ]),
                Polynomial::from_coeffs(vec![
                    Integer::from(-2),
                    Integer::from(0),
                    Integer::from(1),
                ]),
                Polynomial::from_coeffs(vec![
                    Integer::from(-4),
                    Integer::from(0),
                    Integer::from(1),
                ]),
            ),
        ] {
            println!();
            let rpp = root_product_poly(&f, &g);
            println!("f = {}", Polynomial::to_string(&f));
            println!("g = {}", Polynomial::to_string(&g));
            println!(
                "exp = {}    exp_factored = {:?}",
                Polynomial::to_string(&exp),
                Integer::structure()
                    .polynomial_ring()
                    .factorize_by_kroneckers_method(exp.clone(), Integer::factor)
            );
            println!(
                "rpp = {}    rpp_factored = {:?}",
                Polynomial::to_string(&rpp),
                Integer::structure()
                    .polynomial_ring()
                    .factorize_by_kroneckers_method(rpp.clone(), Integer::factor)
            );
            assert!(Polynomial::are_associate(&exp, &rpp));
        }
    }
}
