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
use algebraeon_rings::isolated_algebraic::RealAlgebraic;
use algebraeon_rings::structure::PositiveRealNthRootSignature;
use algebraeon_rings::structure::RingSignature;
use algebraeon_sets::structure::*;
use rand::Rng;

fn main() {
    // let space = AffineSpace::new_linear(Rational::structure(), 2);
    // let p1 = Vector::new(space.clone(), vec![Rational::from(0), Rational::from(0)]);
    // let p2 = Vector::new(space.clone(), vec![Rational::from(1), Rational::from(0)]);
    // let p3 = Vector::new(space.clone(), vec![Rational::from(0), Rational::from(1)]);

    // let s1 = Simplex::new(space.clone(), vec![p1.clone()]).unwrap();
    // let s2 = Simplex::new(space.clone(), vec![p1.clone(), p2.clone()]).unwrap();
    // let s3 = Simplex::new(space.clone(), vec![p1.clone(), p2.clone(), p3.clone()]).unwrap();

    let field = RealAlgebraic::structure_ref();

    let space = AffineSpace::new_linear(field, 2);

    let sqrt2 = field
        .nth_root(&field.from_int(Integer::from(2)), 2)
        .unwrap();
    let sqrt3 = field
        .nth_root(&field.from_int(Integer::from(3)), 2)
        .unwrap();

    let a = space
        .convex_hull(vec![
            space.vector([
                field.from_int(Integer::from(0)),
                field.from_int(Integer::from(0)),
            ]),
            space.vector([
                field.from_int(Integer::from(0)),
                field.from_int(Integer::from(1)),
            ]),
            space.vector([sqrt3.clone(), field.from_int(Integer::from(0))]),
        ])
        .to_simplicial_complex()
        .forget_labels();

    let b = space
        .convex_hull(vec![
            space.vector([
                field.from_int(Integer::from(0)),
                field.from_int(Integer::from(0)),
            ]),
            space.vector([sqrt2.clone(), field.from_int(Integer::from(1))]),
            space.vector([sqrt2.clone(), field.from_int(Integer::from(0))]),
        ])
        .to_simplicial_complex()
        .into_forget_labels();

    let x = a.union(&b);

    for spx in x.simplexes() {
        for pt in spx.points() {
            println!("{} {}", pt.coordinate(0), pt.coordinate(1));
        }
    }

    let mut canvas = Canvas2D::new(Box::new(MouseWheelZoomCamera::new()));
    canvas.plot_shapes(
        [Shape::SetThickness(0.3)]
            .into_iter()
            .chain(simplicial_complex_shapes(
                &Colour::green(),
                &Colour::green().darken(),
                0.5,
                &x,
            )),
    );
    canvas.run();
}
