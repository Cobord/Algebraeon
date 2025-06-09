use super::*;
use algebraeon_rings::matrix::{Matrix, MatrixStructure};
use simplexes::{OrientedHyperplane, OrientedSimplex, Simplex};

#[derive(Debug, Clone)]
pub struct EmbeddedAffineSubspace<
    FS: OrderedRingSignature + FieldSignature,
    SP: Borrow<AffineSpace<FS>> + Clone,
    ESP: Borrow<AffineSpace<FS>> + Clone,
> {
    // The ordered_field of ambient_space and subspace must match
    ambient_space: SP,
    embedded_space: ESP,
    /*
    these vectors must be equal in length to the affine dimension of subspace
    they define the embedding of subspace into ambient space
        if they are empty then they define the empty embedding
        if there is one vector then it defines the location of the embedded point
        if there are vectors [v0, v1, v2, ..., vn] then the embedding sends (a1, a2, ..., an) in subspace to v0 + a1*v1, v0 + a2*v2, ..., v0 + an*vn in ambient space
    */
    embedding_points: Vec<Vector<FS, SP>>,
}

impl<
    FS: OrderedRingSignature + FieldSignature,
    SP: Borrow<AffineSpace<FS>> + Clone,
    ESP: Borrow<AffineSpace<FS>> + From<AffineSpace<FS>> + Clone,
> EmbeddedAffineSubspace<FS, SP, ESP>
{
    #[allow(clippy::type_complexity)]
    pub fn new_affine_span(
        ambient_space: SP,
        points: Vec<Vector<FS, SP>>,
    ) -> Result<(Self, Vec<Vector<FS, ESP>>), &'static str> {
        for point in &points {
            debug_assert_eq!(point.ambient_space().borrow(), ambient_space.borrow());
        }
        if !ambient_space
            .borrow()
            .are_points_affine_independent(points.iter().collect())
        {
            return Err("Affine embedding points must be affine independent");
        }
        let ordered_field = ambient_space.borrow().ordered_field();
        let embedded_space: ESP =
            AffineSpace::new_affine(ordered_field.clone(), points.len()).into();
        let n = points.len();
        let embedded_pts = (0..n)
            .map(|i| {
                Vector::construct(embedded_space.clone(), |j| {
                    if i == 0 {
                        ordered_field.zero()
                    } else if i == j + 1 {
                        ordered_field.one()
                    } else {
                        ordered_field.zero()
                    }
                })
            })
            .collect();
        Ok((
            Self {
                ambient_space,
                embedded_space,
                embedding_points: points,
            },
            embedded_pts,
        ))
    }

    pub fn new_empty(ambient_space: SP) -> Self {
        Self::new_affine_span(ambient_space, vec![]).unwrap().0
    }
}

impl<FS: OrderedRingSignature + FieldSignature, SP: Borrow<AffineSpace<FS>> + Clone>
    EmbeddedAffineSubspace<FS, SP, AffineSpace<FS>>
{
    #[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
    pub fn new(
        ambient_space: SP,
        root: Vector<FS, SP>,
        span: Vec<Vector<FS, SP>>,
    ) -> Result<(Self, Vec<Vector<FS, AffineSpace<FS>>>), &'static str> {
        let mut points = vec![root.clone()];
        points.extend(span.iter().map(|vec| &root + vec));
        Self::new_affine_span(ambient_space, points)
    }

    pub fn new_affine_span_linearly_dependent(
        ambient_space: SP,
        points: Vec<&Vector<FS, SP>>,
    ) -> Self {
        if points.is_empty() {
            Self::new_empty(ambient_space)
        } else {
            let dim = ambient_space.borrow().linear_dimension().unwrap();
            let ordered_field = ambient_space.borrow().ordered_field();
            let mut points = points.into_iter();
            let root = points.next().unwrap();
            let span = points.map(|pt| pt - root).collect::<Vec<_>>();
            //matrix whose columns are pt - root for every other pt in points
            let mat = Matrix::construct(dim, span.len(), |r, c| span[c].coordinate(r).clone());
            let (_, _, _, pivs) =
                MatrixStructure::new(ordered_field.clone()).row_hermite_algorithm(mat);
            Self::new(
                ambient_space,
                root.clone(),
                pivs.into_iter().map(|i| span[i].clone()).collect(),
            )
            .unwrap()
            .0
        }
    }
}

impl<
    FS: OrderedRingSignature + FieldSignature,
    SP: Borrow<AffineSpace<FS>> + Clone,
    ESP: Borrow<AffineSpace<FS>> + Clone,
> EmbeddedAffineSubspace<FS, SP, ESP>
{
    pub fn ordered_field(&self) -> &FS {
        self.ambient_space.borrow().ordered_field()
    }

    pub fn ambient_space(&self) -> SP {
        self.ambient_space.clone()
    }

    pub fn embedded_space(&self) -> ESP {
        self.embedded_space.clone()
    }

    //Let A be the affine subspace and let S be its ambient space
    //Find an affine subspace B of S obtained by linearly extending by pt
    //Return the embeddings (f, g, pt) where
    //  f : A -> B
    //  g : B -> S
    //  pt is pt in B
    #[allow(clippy::type_complexity)]
    pub fn extend_dimension_by_point_unsafe(
        &self,
        pt: Vector<FS, SP>,
    ) -> (
        EmbeddedAffineSubspace<FS, AffineSpace<FS>, ESP>,
        EmbeddedAffineSubspace<FS, SP, AffineSpace<FS>>,
        Vector<FS, AffineSpace<FS>>,
    ) {
        debug_assert_eq!(self.ambient_space.borrow(), pt.ambient_space().borrow());
        debug_assert!(self.unembed_point(&pt).is_none());
        let ordered_field = self.ordered_field();

        let n = self.embedded_space.borrow().affine_dimension();
        let extended_embedded_space = AffineSpace::new_affine(ordered_field.clone(), n + 1);

        (
            EmbeddedAffineSubspace {
                ambient_space: extended_embedded_space.clone(),
                embedded_space: self.embedded_space.clone(),
                // 0, e_1, e_2, ..., e_(n-1)
                embedding_points: {
                    (0..n)
                        .map(|k| {
                            Vector::construct(extended_embedded_space.clone(), |i| {
                                if k == 0 {
                                    ordered_field.zero()
                                } else {
                                    let j = k - 1;
                                    if i == j {
                                        ordered_field.one()
                                    } else {
                                        ordered_field.zero()
                                    }
                                }
                            })
                        })
                        .collect()
                },
            },
            EmbeddedAffineSubspace {
                ambient_space: self.ambient_space.clone(),
                embedded_space: extended_embedded_space.clone(),
                embedding_points: {
                    let mut pts = self.embedding_points.clone();
                    pts.push(pt);
                    pts
                },
            },
            Vector::construct(extended_embedded_space.clone(), |i| {
                if i + 1 == n {
                    ordered_field.one()
                } else {
                    ordered_field.zero()
                }
            }),
        )
    }

    #[allow(clippy::type_complexity)]
    pub fn get_root_and_span(&self) -> Option<(Vector<FS, SP>, Vec<Vector<FS, SP>>)> {
        let mut points = self.embedding_points.iter();
        let root = points.next()?;
        let span = points.map(|pt| pt - root).collect::<Vec<_>>();
        Some((root.clone(), span))
    }

    pub fn get_embedding_points(&self) -> &Vec<Vector<FS, SP>> {
        &self.embedding_points
    }

    pub fn embed_point(&self, pt: &Vector<FS, ESP>) -> Vector<FS, SP> {
        assert_eq!(pt.ambient_space().borrow(), self.embedded_space.borrow());
        let (root, span) = self.get_root_and_span().unwrap(); //pt exists in the embedded space, so the embedded space is non-empty, so has a root and span
        let mut total = root.clone();
        for (i, vec) in span.iter().enumerate() {
            total += &vec.scalar_mul(pt.coordinate(i));
        }
        total
    }

    pub fn embed_simplex(&self, spx: &Simplex<FS, ESP>) -> Simplex<FS, SP> {
        Simplex::new(
            self.ambient_space(),
            spx.points().iter().map(|p| self.embed_point(p)).collect(),
        )
        .unwrap()
    }

    pub fn unembed_point(&self, pt: &Vector<FS, SP>) -> Option<Vector<FS, ESP>> {
        assert_eq!(pt.ambient_space().borrow(), self.ambient_space.borrow());
        match self.get_root_and_span() {
            Some((root, span)) => {
                //solve root + x * basis = v for x
                let y = (pt - &root).into_coordinates();
                let basis_matrix = self
                    .ambient_space
                    .borrow()
                    .cols_from_vectors(span.iter().collect());
                let x = MatrixStructure::new(self.ambient_space.borrow().ordered_field().clone())
                    .col_solve(basis_matrix, &y);
                Some(Vector::new(self.embedded_space(), x?))
            }
            None => None,
        }
    }

    pub fn unembed_simplex(&self, spx: &Simplex<FS, SP>) -> Option<Simplex<FS, ESP>> {
        let mut pts = vec![];
        for embedded_pt in spx.points() {
            match self.unembed_point(embedded_pt) {
                Some(pt) => {
                    pts.push(pt);
                }
                None => {
                    return None;
                }
            }
        }
        Some(Simplex::new(self.embedded_space(), pts).unwrap())
    }

    pub fn as_hyperplane_intersection(&self) -> Option<Vec<OrientedHyperplane<FS, SP>>> {
        let ambient_space = self.ambient_space();
        let ordered_field = ambient_space.borrow().ordered_field();
        match self.get_root_and_span() {
            Some((root, span)) => {
                let dim_amb = ambient_space.borrow().linear_dimension().unwrap();
                let dim_ss = span.len();
                // step 1: extend span to a basis by appending a subset of the elementary basis vectors
                //first columns = span, remaining columns = identity matrix
                let mat = Matrix::construct(dim_amb, dim_ss + dim_amb, |r, c| {
                    if c < dim_ss {
                        span[c].coordinate(r).clone()
                    } else {
                        let c = c - dim_ss;
                        if r == c {
                            ordered_field.one()
                        } else {
                            ordered_field.zero()
                        }
                    }
                });
                let (_, _, _, pivs) =
                    MatrixStructure::new(ordered_field.clone()).row_hermite_algorithm(mat);
                debug_assert_eq!(pivs.len(), dim_amb);
                #[allow(clippy::needless_range_loop)]
                for i in 0..dim_ss {
                    debug_assert_eq!(pivs[i], i); //span is linearly independent so we expect this
                }
                let extension_elementary_basis_vectors = (dim_ss..dim_amb)
                    .map(|i| pivs[i] - dim_ss)
                    .collect::<Vec<_>>();

                // step 2: take the hyperplanes formed by removing exactly one of each of the added elementary basis vectors at a time
                let hyperplanes = (0..extension_elementary_basis_vectors.len())
                    .map(|i| {
                        let ref_point = {
                            //root + e_k
                            let k = extension_elementary_basis_vectors[i];
                            Vector::construct(ambient_space.clone(), |l| {
                                ordered_field.add(
                                    root.coordinate(l),
                                    &if l == k {
                                        ordered_field.one()
                                    } else {
                                        ordered_field.zero()
                                    },
                                )
                            })
                        };
                        OrientedSimplex::new_with_positive_point(
                            ambient_space.clone(),
                            {
                                let mut points = vec![root.clone()];
                                for s in &span {
                                    points.push(&root + s);
                                }
                                #[allow(clippy::needless_range_loop)]
                                for j in 0..extension_elementary_basis_vectors.len() {
                                    if i != j {
                                        let k = extension_elementary_basis_vectors[j];
                                        //push root + e_k
                                        points.push(Vector::construct(
                                            ambient_space.clone(),
                                            |l| {
                                                ordered_field.add(root.coordinate(l), &{
                                                    if l == k {
                                                        ordered_field.one()
                                                    } else {
                                                        ordered_field.zero()
                                                    }
                                                })
                                            },
                                        ));
                                    }
                                }
                                points
                            },
                            &ref_point,
                        )
                        .unwrap()
                        .into_oriented_hyperplane()
                    })
                    .collect::<Vec<_>>();
                debug_assert_eq!(hyperplanes.len(), dim_amb - dim_ss);
                Some(hyperplanes)
            }
            None => None,
        }
    }
}

pub fn compose_affine_embeddings<
    FS: OrderedRingSignature + FieldSignature,
    SPA: Borrow<AffineSpace<FS>> + Clone,
    SPB: Borrow<AffineSpace<FS>> + Clone,
    SPC: Borrow<AffineSpace<FS>> + Clone,
>(
    _a_to_b: EmbeddedAffineSubspace<FS, SPB, SPA>,
    _b_to_c: EmbeddedAffineSubspace<FS, SPC, SPB>,
) -> EmbeddedAffineSubspace<FS, SPC, SPA> {
    todo!() // call b_to_c.embed on the defining points of a_to_b
}

#[cfg(test)]
mod tests {
    use algebraeon_nzq::Rational;
    use algebraeon_sets::structure::*;

    use super::*;

    #[test]
    fn make_affine_subspace() {
        let space = AffineSpace::new_linear(Rational::structure(), 3);
        let v1 = Vector::new(
            &space,
            vec![Rational::from(1), Rational::from(1), Rational::from(1)],
        );
        let v2 = Vector::new(
            &space,
            vec![Rational::from(1), Rational::from(0), Rational::from(0)],
        );
        let v3 = Vector::new(
            &space,
            vec![Rational::from(0), Rational::from(1), Rational::from(0)],
        );
        let s = EmbeddedAffineSubspace::new(&space, v1, vec![v2, v3]);
        s.unwrap();

        let space = AffineSpace::new_linear(Rational::structure(), 3);
        let v1 = Vector::new(
            &space,
            vec![Rational::from(1), Rational::from(1), Rational::from(1)],
        );
        let v2 = Vector::new(
            &space,
            vec![Rational::from(1), Rational::from(2), Rational::from(0)],
        );
        let v3 = Vector::new(
            &space,
            vec![Rational::from(-2), Rational::from(-4), Rational::from(0)],
        );
        let s = EmbeddedAffineSubspace::new(&space, v1, vec![v2, v3]);
        assert!(s.is_err());
    }

    #[test]
    fn affine_subspace_embed_and_unembed() {
        //1d embedded in 2d
        {
            let plane = AffineSpace::new_linear(Rational::structure(), 2);
            //the line x + y = 2
            let (line, _) = EmbeddedAffineSubspace::new(
                &plane,
                Vector::new(&plane, vec![Rational::from(1), Rational::from(1)]),
                vec![Vector::new(
                    &plane,
                    vec![Rational::from(1), Rational::from(-1)],
                )],
            )
            .unwrap();

            assert_eq!(
                line.embed_point(&Vector::new(
                    line.embedded_space(),
                    vec![Rational::from(-3)],
                )),
                Vector::new(&plane, vec![Rational::from(-2), Rational::from(4)])
            );

            assert_eq!(
                line.unembed_point(&Vector::new(
                    &plane,
                    vec![Rational::from(-1), Rational::from(3)],
                )),
                Some(Vector::new(line.embedded_space(), vec![Rational::from(-2)],))
            );

            assert_eq!(
                line.unembed_point(&Vector::new(
                    &plane,
                    vec![Rational::from(1), Rational::from(2)],
                )),
                None
            );
        }

        //2d embedded in 3d
        {
            let space = AffineSpace::new_linear(Rational::structure(), 3);
            let (plane, _) = EmbeddedAffineSubspace::new(
                &space,
                Vector::new(
                    &space,
                    vec![Rational::from(3), Rational::from(1), Rational::from(2)],
                ),
                vec![
                    Vector::new(
                        &space,
                        vec![Rational::from(4), Rational::from(2), Rational::from(1)],
                    ),
                    Vector::new(
                        &space,
                        vec![Rational::from(1), Rational::from(-1), Rational::from(2)],
                    ),
                ],
            )
            .unwrap();

            assert_eq!(
                plane.embed_point(&Vector::new(
                    plane.embedded_space(),
                    vec![Rational::from(-3), Rational::from(2)],
                )),
                Vector::new(
                    &space,
                    vec![Rational::from(-7), Rational::from(-7), Rational::from(3)]
                )
            );

            assert_eq!(
                plane.unembed_point(&Vector::new(
                    &space,
                    vec![Rational::from(0), Rational::from(-2), Rational::from(3)],
                )),
                Some(Vector::new(
                    plane.embedded_space(),
                    vec![Rational::from(-1), Rational::from(1)],
                ))
            );

            assert_eq!(
                plane.unembed_point(&Vector::new(
                    &space,
                    vec![Rational::from(1), Rational::from(2), Rational::from(2)],
                )),
                None
            );
        }
    }

    #[test]
    fn extend_by_point_embedding_composition() {
        let space = AffineSpace::new_linear(Rational::structure(), 4);
        let v1 = Vector::new(
            &space,
            vec![
                Rational::from(1),
                Rational::from(2),
                Rational::from(1),
                Rational::from(1),
            ],
        );
        let v2 = Vector::new(
            &space,
            vec![
                Rational::from(1),
                Rational::from(-2),
                Rational::from(2),
                Rational::from(0),
            ],
        );
        let v3 = Vector::new(
            &space,
            vec![
                Rational::from(2),
                Rational::from(1),
                Rational::from(0),
                Rational::from(2),
            ],
        );
        let (h, _) = EmbeddedAffineSubspace::new(&space, v1, vec![v2, v3]).unwrap();
        let v4 = Vector::new(
            &space,
            vec![
                Rational::from(0),
                Rational::from(3),
                Rational::from(-2),
                Rational::from(1),
            ],
        );
        let (f, g, v4_inv) = h.extend_dimension_by_point_unsafe(v4.clone());
        assert_eq!(g.embed_point(&v4_inv), v4);

        let x = Vector::new(
            h.embedded_space(),
            vec![Rational::from(5), Rational::from(7)],
        );
        //check that g(f(x)) = h(x)
        assert_eq!(g.embed_point(&f.embed_point(&x)), h.embed_point(&x));
    }
}
