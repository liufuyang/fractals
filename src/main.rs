use newton_factal::math::polynomial::Polynomial;
use newton_factal::{render_image, Field};

fn main() {
    let pol = Polynomial::new(vec![-1, 0, 0, 1]);
    let field = Field {
        source: (0, 0),
        ssize: 512,

        target: (-5., -5.),
        tsize: 10.0,
    };
    let image = render_image(pol, field);
    image.save("output.png").unwrap();
}
