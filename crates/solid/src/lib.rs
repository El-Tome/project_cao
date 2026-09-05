//! Les volumes : un maillage de polygones, l'extrusion d'une aire d'esquisse
//! en prisme, et les opérations booléennes qui ajoutent ou enlèvent de la
//! matière. Ni rendu ni interface, comme `cao_sketch`.

mod boolean;
mod mesh;

pub use mesh::{FaceHit, Mesh, Polygon, prism, revolution};
