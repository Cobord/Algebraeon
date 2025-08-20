use super::*;
use crate::{
    ambient_space::AffineSpace, partial_simplicial_complex::LabelledPartialSimplicialComplex,
    simplex::Simplex, simplicial_disjoint_union::LabelledSimplicialDisjointUnion,
};
use std::collections::{HashMap, HashSet};

/// A collection of disjoint simplices labelled by T
pub trait LabelledSimplexCollection<
    'f,
    FS: OrderedRingSignature + FieldSignature + 'f,
    T: Eq + Clone + Send + Sync,
>: Sized where
    FS::Set: Hash,
{
    type WithLabel<S: Eq + Clone + Send + Sync>: LabelledSimplexCollection<'f, FS, S>;
    type SubsetType: LabelledSimplexCollection<'f, FS, T>;

    fn try_new(
        ambient_space: AffineSpace<'f, FS>,
        simplexes: HashSet<Simplex<'f, FS>>,
    ) -> Result<Self::WithLabel<()>, &'static str> {
        Self::WithLabel::<()>::try_new_labelled(
            ambient_space,
            simplexes.into_iter().map(|spx| (spx, ())).collect(),
        )
    }
    fn new_unchecked(
        ambient_space: AffineSpace<'f, FS>,
        simplexes: HashSet<Simplex<'f, FS>>,
    ) -> Self::WithLabel<()> {
        Self::WithLabel::<()>::new_labelled_unchecked(
            ambient_space,
            simplexes.into_iter().map(|spx| (spx, ())).collect(),
        )
    }

    fn try_new_labelled(
        ambient_space: AffineSpace<'f, FS>,
        simplexes: HashMap<Simplex<'f, FS>, T>,
    ) -> Result<Self, &'static str>;
    fn new_labelled_unchecked(
        ambient_space: AffineSpace<'f, FS>,
        simplexes: HashMap<Simplex<'f, FS>, T>,
    ) -> Self;

    fn ambient_space(&self) -> AffineSpace<'f, FS>;

    fn simplexes<'a>(&'a self) -> HashSet<&'a Simplex<'f, FS>>
    where
        T: 'a,
    {
        self.labelled_simplexes().into_keys().collect()
    }
    fn into_simplexes(self) -> HashSet<Simplex<'f, FS>> {
        self.into_labelled_simplexes().into_keys().collect()
    }

    fn labelled_simplexes(&self) -> HashMap<&Simplex<'f, FS>, &T>;
    fn into_labelled_simplexes(self) -> HashMap<Simplex<'f, FS>, T>;

    fn subset_by_label(
        &self,
        label: &T,
    ) -> <Self::SubsetType as LabelledSimplexCollection<'f, FS, T>>::WithLabel<()> {
        self.subset_by_filter(|spx_label| spx_label == label)
            .forget_labels()
    }
    fn into_subset_by_label(
        self,
        label: &T,
    ) -> <Self::SubsetType as LabelledSimplexCollection<'f, FS, T>>::WithLabel<()> {
        self.into_subset_by_filter(|spx_label| spx_label == label)
            .forget_labels()
    }
    fn subset_by_filter(&self, f: impl Fn(&T) -> bool) -> Self::SubsetType {
        Self::SubsetType::new_labelled_unchecked(
            self.ambient_space(),
            self.labelled_simplexes()
                .into_iter()
                .filter(|(_spx, label)| f(label))
                .map(|(spx, label)| (spx.clone(), label.clone()))
                .collect(),
        )
    }
    fn into_subset_by_filter(self, f: impl Fn(&T) -> bool) -> Self::SubsetType {
        Self::SubsetType::new_labelled_unchecked(
            self.ambient_space(),
            self.into_labelled_simplexes()
                .into_iter()
                .filter(|(_spx, label)| f(label))
                .collect(),
        )
    }

    fn into_partial_simplicial_complex(self) -> LabelledPartialSimplicialComplex<'f, FS, T>;
    fn to_partial_simplicial_complex(&self) -> LabelledPartialSimplicialComplex<'f, FS, T>;

    fn into_simplicial_disjoint_union(self) -> LabelledSimplicialDisjointUnion<'f, FS, T>;
    fn to_simplicial_disjoint_union(&self) -> LabelledSimplicialDisjointUnion<'f, FS, T>;

    fn apply_label_function<S: Eq + Clone + Send + Sync>(
        &self,
        f: impl Fn(&T) -> S,
    ) -> Self::WithLabel<S> {
        LabelledSimplexCollection::new_labelled_unchecked(
            self.ambient_space(),
            self.labelled_simplexes()
                .into_iter()
                .map(|(spx, label)| (spx.clone(), f(label)))
                .collect(),
        )
    }
    fn into_apply_label_function<S: Eq + Clone + Send + Sync>(
        self,
        f: impl Fn(T) -> S,
    ) -> Self::WithLabel<S> {
        LabelledSimplexCollection::new_labelled_unchecked(
            self.ambient_space(),
            self.into_labelled_simplexes()
                .into_iter()
                .map(|(spx, label)| (spx, f(label)))
                .collect(),
        )
    }
    fn forget_labels(&self) -> Self::WithLabel<()> {
        self.apply_label_function(|_| ())
    }
    fn into_forget_labels(self) -> Self::WithLabel<()> {
        self.into_apply_label_function(|_| ())
    }

    fn common_label<'a>(
        &'a self,
        simplexes: impl Iterator<Item = &'a Simplex<'f, FS>>,
    ) -> Option<&'a T>
    where
        FS: 'a,
        'f: 'a,
        AffineSpace<'f, FS>: 'a,
    {
        let mut label = None;
        for spx in simplexes {
            let spx_label = *self.labelled_simplexes().get(&spx).unwrap();
            match label {
                Some(label) => {
                    if label != spx_label {
                        return None;
                    }
                }
                None => {
                    label = Some(spx_label);
                }
            }
        }
        label
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteriorOrBoundary {
    Interior,
    Boundary,
}

pub trait InteriorOrBoundarySimplexCollection<'f, FS: OrderedRingSignature + FieldSignature + 'f>:
    LabelledSimplexCollection<'f, FS, InteriorOrBoundary>
where
    FS::Set: Hash,
{
    fn interior(
        &self,
    ) -> <Self::SubsetType as LabelledSimplexCollection<'f, FS, InteriorOrBoundary>>::WithLabel<()>
    {
        self.subset_by_label(&InteriorOrBoundary::Interior)
    }

    fn boundary(
        &self,
    ) -> <Self::SubsetType as LabelledSimplexCollection<'f, FS, InteriorOrBoundary>>::WithLabel<()>
    {
        self.subset_by_label(&InteriorOrBoundary::Boundary)
    }
}

impl<
    'f,
    FS: OrderedRingSignature + FieldSignature + 'f,
    S: LabelledSimplexCollection<'f, FS, InteriorOrBoundary>,
> InteriorOrBoundarySimplexCollection<'f, FS> for S
where
    FS::Set: Hash,
{
}
