#![allow(dead_code, warnings, unused)]

use algebraeon_drawing::canvas::Canvas;
use algebraeon_drawing::canvas2d::Canvas2D;
use algebraeon_drawing::canvas2d::MouseWheelZoomCamera;
use algebraeon_drawing::canvas2d::shapes::Shape;
use algebraeon_drawing::canvas2d::shapes::simplicial_complex_shapes;
use algebraeon_drawing::colour::Colour;
use algebraeon_geometry::ambient_space::AffineSpace;
use algebraeon_geometry::boolean_operations::Union;
use algebraeon_geometry::convex_hull::ConvexHull;
use algebraeon_geometry::simplex_collection::LabelledSimplexCollection;
use algebraeon_geometry::simplicial_disjoint_union::LabelledSimplicialDisjointUnion;
use algebraeon_geometry::vector::Vector;
use algebraeon_geometry::*;
use algebraeon_nzq::*;
use algebraeon_rings::structure::*;
use algebraeon_sets::structure::*;
use rand::Rng;
use std::rc::Rc;

fn main() {
    // let space = AffineSpace::new_linear(Rational::structure(), 2);
    // let p1 = Vector::new(&space, vec![Rational::from(0), Rational::from(0)]);
    // let p2 = Vector::new(&space, vec![Rational::from(1), Rational::from(0)]);
    // let p3 = Vector::new(&space, vec![Rational::from(0), Rational::from(1)]);

    // let s1 = Simplex::new(&space, vec![p1.clone()]).unwrap();
    // let s2 = Simplex::new(&space, vec![p1.clone(), p2.clone()]).unwrap();
    // let s3 = Simplex::new(&space, vec![p1.clone(), p2.clone(), p3.clone()]).unwrap();

    let space = AffineSpace::new_linear(Rational::structure_ref(), 2);

    fn random_point(
        space: AffineSpace<'static, RationalCanonicalStructure>,
        rad: f64,
    ) -> Vector<'static, RationalCanonicalStructure> {
        let mut rng = rand::thread_rng();
        Vector::construct(space, |i| {
            Rational::from_f64_approx(rng.gen_range(-rad..rad)).approximate(&Natural::from(3u64))
        })
    }

    // let pt1 = Vector::new( space, vec![Rational::from(0), Rational::from(0)]);
    // let pt2 = Vector::new( space, vec![Rational::from(0), Rational::from(-1)]);
    // let pt3 = Vector::new( space, vec![Rational::from(0), Rational::from(1)]);
    // let pt4 = Vector::new( space, vec![Rational::from(1), Rational::from(0)]);

    // let spx1 = Simplex::new( space, vec![pt1]).unwrap();
    // let spx2 = Simplex::new( space, vec![pt2, pt3, pt4]).unwrap();

    // let VennResult {
    //     left: a,
    //     middle: b,
    //     right: c,
    // } = spx1.venn(&spx2);

    let n = 12;

    let ch1 = space.convex_hull(
        (0..n)
            .map(|i| random_point(space, (i + 1) as f64))
            .collect(),
    );
    let ch2 = space.convex_hull(
        (0..n)
            .map(|i| random_point(space, (i + 1) as f64))
            .collect(),
    );

    // let ch3 = ch1.intersect(&ch2);

    let sc1 = ch1.to_simplicial_complex().into_forget_labels();
    let sc2 = ch2.to_simplicial_complex().into_forget_labels();
    // let sc3 = ch3.as_simplicial_complex().entire;

    // let VennResult {
    //     left: a,
    //     middle: b,
    //     right: c,0

    // }

    let sc4 = sc1.union(&sc2);

    let mut canvas = Canvas2D::new(Box::new(MouseWheelZoomCamera::new()));
    canvas.plot_shapes(
        [Shape::SetThickness(0.3)]
            .into_iter()
            .chain(simplicial_complex_shapes(
                &Colour::magenta(),
                &Colour::magenta().darken(),
                0.6,
                &sc1,
            ))
            .chain(simplicial_complex_shapes(
                &Colour::cyan(),
                &Colour::cyan().darken(),
                0.6,
                &sc2,
            ))
            .chain(simplicial_complex_shapes(
                &Colour::green(),
                &Colour::green().darken(),
                0.3,
                &sc4,
            )),
    );
    canvas.run();
}
