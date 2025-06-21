use super::*;
use crate::matrix::{Matrix, MatrixStructure};
use crate::polynomial::Polynomial;
use algebraeon_sets::structure::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::marker::PhantomData;

mod range_module {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct RingHomomorphismRangeModuleStructure<
        'h,
        Domain: RingSignature,
        Range: RingSignature,
        Hom: RingHomomorphism<Domain, Range>,
    > {
        _domain: PhantomData<Domain>,
        _range: PhantomData<Range>,
        hom: &'h Hom,
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>>
        RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
        fn new(hom: &'h Hom) -> Self {
            Self {
                _domain: PhantomData,
                _range: PhantomData,
                hom,
            }
        }

        pub fn module(&self) -> &Range {
            self.hom.range()
        }

        pub fn homomorphism(&self) -> &'h Hom {
            self.hom
        }
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>>
        PartialEq for RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
        fn eq(&self, other: &Self) -> bool {
            std::ptr::eq(self.hom, other.hom)
        }
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>> Eq
        for RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>>
        Signature for RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>>
        SetSignature for RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
        type Set = Range::Set;

        fn is_element(&self, x: &Self::Set) -> Result<(), String> {
            self.hom.range().is_element(x)
        }
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>>
        EqSignature for RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
        fn equal(&self, a: &Self::Set, b: &Self::Set) -> bool {
            self.hom.range().equal(a, b)
        }
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>>
        AdditiveMonoidSignature for RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
        fn zero(&self) -> Self::Set {
            self.hom.range().zero()
        }

        fn add(&self, a: &Self::Set, b: &Self::Set) -> Self::Set {
            self.hom.range().add(a, b)
        }
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>>
        AdditiveGroupSignature for RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
        fn neg(&self, a: &Self::Set) -> Self::Set {
            self.hom.range().neg(a)
        }
    }

    impl<'h, Domain: RingSignature, Range: RingSignature, Hom: RingHomomorphism<Domain, Range>>
        SemiModuleSignature<Domain>
        for RingHomomorphismRangeModuleStructure<'h, Domain, Range, Hom>
    {
        fn ring(&self) -> &Domain {
            self.hom.domain()
        }

        fn scalar_mul(&self, a: &Self::Set, x: &Domain::Set) -> Self::Set {
            self.hom.range().mul(&self.hom.image(x), a)
        }
    }

    pub trait RingHomomorphism<Domain: RingSignature, Range: RingSignature>:
        Function<Domain, Range>
    {
        fn range_module_structure<'h>(
            &'h self,
        ) -> RingHomomorphismRangeModuleStructure<'h, Domain, Range, Self> {
            RingHomomorphismRangeModuleStructure::new(self)
        }
    }

    impl<
        A: RingSignature,
        B: RingSignature,
        C: RingSignature,
        AB: RingHomomorphism<A, B>,
        BC: RingHomomorphism<B, C>,
    > RingHomomorphism<A, C> for CompositionMorphism<A, B, C, AB, BC>
    {
    }
}
pub use range_module::*;

mod principal_subring_inclusion {
    use super::*;
    use algebraeon_nzq::*;

    /// The unique ring homomorphism Z -> R of the integers into any ring R
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PrincipalSubringInclusion<Ring: RingSignature, RingB: BorrowedStructure<Ring>> {
        _ring: PhantomData<Ring>,
        ring: RingB,
    }

    impl<Ring: RingSignature, RingB: BorrowedStructure<Ring>> PrincipalSubringInclusion<Ring, RingB> {
        pub fn new(ring: RingB) -> Self {
            Self {
                _ring: PhantomData,
                ring,
            }
        }
    }

    impl<Ring: RingSignature, RingB: BorrowedStructure<Ring>>
        Morphism<IntegerCanonicalStructure, Ring> for PrincipalSubringInclusion<Ring, RingB>
    {
        fn domain(&self) -> &IntegerCanonicalStructure {
            Integer::structure_ref()
        }

        fn range(&self) -> &Ring {
            self.ring.borrow()
        }
    }

    impl<Ring: RingSignature, RingB: BorrowedStructure<Ring>>
        Function<IntegerCanonicalStructure, Ring> for PrincipalSubringInclusion<Ring, RingB>
    {
        fn image(&self, x: &Integer) -> <Ring as SetSignature>::Set {
            self.range().from_int(x)
        }
    }

    impl<Ring: CharZeroRingSignature, RingB: BorrowedStructure<Ring>>
        InjectiveFunction<IntegerCanonicalStructure, Ring>
        for PrincipalSubringInclusion<Ring, RingB>
    {
        fn try_preimage(&self, x: &<Ring as SetSignature>::Set) -> Option<Integer> {
            self.range().try_to_int(x)
        }
    }

    impl<Ring: RingSignature, RingB: BorrowedStructure<Ring>>
        RingHomomorphism<IntegerCanonicalStructure, Ring>
        for PrincipalSubringInclusion<Ring, RingB>
    {
    }

    /// The unique field embedding Q -> K of the rationals into any field of characteristic zero
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PrincipalRationalSubfieldInclusion<
        Field: CharZeroFieldSignature,
        FieldB: BorrowedStructure<Field>,
    > {
        _field: PhantomData<Field>,
        field: FieldB,
    }

    impl<Field: CharZeroFieldSignature, FieldB: BorrowedStructure<Field>>
        PrincipalRationalSubfieldInclusion<Field, FieldB>
    {
        pub fn new(field: FieldB) -> Self {
            Self {
                _field: PhantomData,
                field,
            }
        }
    }

    impl<Field: CharZeroFieldSignature, FieldB: BorrowedStructure<Field>>
        Morphism<RationalCanonicalStructure, Field>
        for PrincipalRationalSubfieldInclusion<Field, FieldB>
    {
        fn domain(&self) -> &RationalCanonicalStructure {
            Rational::structure_ref()
        }

        fn range(&self) -> &Field {
            self.field.borrow()
        }
    }

    impl<Field: CharZeroFieldSignature, FieldB: BorrowedStructure<Field>>
        Function<RationalCanonicalStructure, Field>
        for PrincipalRationalSubfieldInclusion<Field, FieldB>
    {
        fn image(&self, x: &Rational) -> <Field as SetSignature>::Set {
            self.range().from_rat(x).unwrap()
        }
    }

    impl<Field: CharZeroFieldSignature, FieldB: BorrowedStructure<Field>>
        InjectiveFunction<RationalCanonicalStructure, Field>
        for PrincipalRationalSubfieldInclusion<Field, FieldB>
    {
        fn try_preimage(&self, x: &<Field as SetSignature>::Set) -> Option<Rational> {
            self.range().try_to_rat(x)
        }
    }

    impl<Field: CharZeroFieldSignature, FieldB: BorrowedStructure<Field>>
        RingHomomorphism<RationalCanonicalStructure, Field>
        for PrincipalRationalSubfieldInclusion<Field, FieldB>
    {
    }
}
pub use principal_subring_inclusion::*;

/// The inclusion of an integral domain into its field of fractions
pub trait FieldOfFractionsInclusion<Ring: RingSignature, Field: FieldSignature>:
    RingHomomorphism<Ring, Field> + InjectiveFunction<Ring, Field>
{
    fn numerator_and_denominator(&self, a: &Field::Set) -> (Ring::Set, Ring::Set);
    fn numerator(&self, a: &Field::Set) -> Ring::Set {
        self.numerator_and_denominator(a).0
    }
    fn denominator(&self, a: &Field::Set) -> Ring::Set {
        self.numerator_and_denominator(a).1
    }
}

/// An injective homomorphism A -> B of integral domains where there is a way to get all roots in B of a polynomial over A
pub trait IntegralDomainExtensionAllPolynomialRoots<
    A: IntegralDomainSignature,
    B: IntegralDomainSignature,
>: RingHomomorphism<A, B> + InjectiveFunction<A, B>
{
    /// Return all roots of the polynomial in B with duplicate elements according to multiplicity
    fn all_roots(&self, polynomial: &Polynomial<A::Set>) -> Vec<B::Set>;
}

/// A ring extension Z -> R such that R is a finitely free Z-module
pub trait FiniteRankFreeRingExtension<Z: RingSignature, R: RingSignature>:
    RingHomomorphism<Z, R> + InjectiveFunction<Z, R>
{
    fn degree(&self) -> usize;

    /// matrix representing column vector multiplication by `a` on the left
    fn col_multiplication_matrix(&self, a: &R::Set) -> Matrix<Z::Set>;

    /// matrix representing row vector multiplication by `a` on the right
    fn row_multiplication_matrix(&self, a: &R::Set) -> Matrix<Z::Set>;
}

impl<Z: RingSignature, R: RingSignature, Hom: RingHomomorphism<Z, R> + InjectiveFunction<Z, R>>
    FiniteRankFreeRingExtension<Z, R> for Hom
where
    for<'h> RingHomomorphismRangeModuleStructure<'h, Z, R, Self>:
        FinitelyFreeModuleSignature<Z, Set = R::Set>,
    for<'h> <RingHomomorphismRangeModuleStructure<'h, Z, R, Self> as FreeModuleSignature<Z>>::Basis:
        FiniteSetSignature,
{
    fn degree(&self) -> usize {
        self.range_module_structure().rank()
    }

    fn col_multiplication_matrix(&self, a: &R::Set) -> Matrix<Z::Set> {
        let basis = self.range_module_structure().basis_vecs();
        Matrix::from_cols(
            (0..self.degree())
                .map(|i| {
                    self.range_module_structure()
                        .to_vec(&self.range().mul(a, &basis[i]))
                })
                .collect(),
        )
    }

    fn row_multiplication_matrix(&self, a: &R::Set) -> Matrix<Z::Set> {
        self.col_multiplication_matrix(a).transpose()
    }
}

/// A finite dimensional field extension F -> K
pub trait FiniteDimensionalFieldExtension<F: FieldSignature, K: FieldSignature>:
    RingHomomorphism<F, K> + InjectiveFunction<F, K> + FiniteRankFreeRingExtension<F, K>
{
    fn norm(&self, a: &K::Set) -> F::Set;

    fn trace(&self, a: &K::Set) -> F::Set;

    /// The monic minimal polynomial of a
    fn min_poly(&self, a: &K::Set) -> Polynomial<F::Set>;

    fn trace_form_matrix(&self, elems: &Vec<K::Set>) -> Matrix<F::Set> {
        let n = self.degree();
        assert_eq!(n, elems.len());
        Matrix::construct(n, n, |r, c| {
            self.trace(&self.range().mul(&elems[r], &elems[c]))
        })
    }

    fn discriminant(&self, elems: &Vec<K::Set>) -> F::Set {
        MatrixStructure::new(self.domain().clone())
            .det(self.trace_form_matrix(elems))
            .unwrap()
    }
}

impl<F: FieldSignature, K: FieldSignature, Hom: RingHomomorphism<F, K> + InjectiveFunction<F, K>>
    FiniteDimensionalFieldExtension<F, K> for Hom
where
    for<'h> RingHomomorphismRangeModuleStructure<'h, F, K, Self>:
        FinitelyFreeModuleSignature<F, Set = K::Set>,
    for<'h> <RingHomomorphismRangeModuleStructure<'h, F, K, Self> as FreeModuleSignature<F>>::Basis:
        FiniteSetSignature,
{
    fn norm(&self, a: &K::Set) -> F::Set {
        MatrixStructure::new(self.domain().clone())
            .det(self.col_multiplication_matrix(a))
            .unwrap()
    }

    fn trace(&self, a: &K::Set) -> F::Set {
        MatrixStructure::new(self.domain().clone())
            .trace(&self.col_multiplication_matrix(a))
            .unwrap()
    }

    fn min_poly(&self, a: &K::Set) -> Polynomial<F::Set> {
        MatrixStructure::new(self.domain().clone())
            .minimal_polynomial(self.col_multiplication_matrix(a))
            .unwrap()
    }
}

/// Represent all ring homomorphisms from `domain` to `range`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RingHomomorphisms<Domain: RingSignature, Range: RingSignature> {
    domain: Domain,
    range: Range,
}

impl<Domain: RingSignature, Range: RingSignature> RingHomomorphisms<Domain, Range> {
    pub fn new(domain: Domain, range: Range) -> Self {
        Self { domain, range }
    }
}

impl<Domain: RingSignature, Range: RingSignature> Signature for RingHomomorphisms<Domain, Range> {}

impl<Domain: FreeRingSignature, Range: RingSignature> SetSignature
    for RingHomomorphisms<Domain, Range>
{
    type Set = HashMap<Domain::Generator, Range::Set>;

    fn is_element(&self, x: &Self::Set) -> Result<(), String> {
        if self.domain.free_generators() != x.keys().cloned().collect::<HashSet<_>>() {
            return Err("missing key".to_string());
        }

        for v in x.values() {
            self.range.is_element(v)?;
        }

        Ok(())
    }
}
