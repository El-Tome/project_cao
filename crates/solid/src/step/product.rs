//! What AP214 sets round a solid for a program to take it as a part: the
//! units and the tolerance it is read in, and the product it is the shape of.

use super::instances::Instances;

/// The context of a representation in millimetres and radians, two points
/// one under `uncertainty`, in the body's own unit.
pub fn context(instances: &mut Instances, uncertainty: f64) -> usize {
    let length = instances.add("(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.))".into());
    let angle = instances.add("(NAMED_UNIT(*)PLANE_ANGLE_UNIT()SI_UNIT($,.RADIAN.))".into());
    let solid_angle =
        instances.add("(NAMED_UNIT(*)SI_UNIT($,.STERADIAN.)SOLID_ANGLE_UNIT())".into());
    let distance = instances.length(uncertainty);
    let uncertainty = instances.add(format!(
        "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE({distance}),#{length},\
         'distance_accuracy_value','')"
    ));
    instances.add(format!(
        "(GEOMETRIC_REPRESENTATION_CONTEXT(3)\
         GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{uncertainty}))\
         GLOBAL_UNIT_ASSIGNED_CONTEXT((#{length},#{angle},#{solid_angle}))\
         REPRESENTATION_CONTEXT('',''))"
    ))
}

pub fn shaped_by(instances: &mut Instances, representation: usize) {
    let application = instances.add("APPLICATION_CONTEXT('automotive_design')".into());
    instances.add(format!(
        "APPLICATION_PROTOCOL_DEFINITION('international_standard','automotive_design',2000,\
         #{application})"
    ));
    let context = instances.add(format!("PRODUCT_CONTEXT('',#{application},'mechanical')"));
    let product = instances.add(format!("PRODUCT('part','part','',(#{context}))"));
    instances.add(format!(
        "PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#{product}))"
    ));
    let formation = instances.add(format!("PRODUCT_DEFINITION_FORMATION('','',#{product})"));
    let context = instances.add(format!(
        "PRODUCT_DEFINITION_CONTEXT('part_definition',#{application},'design')"
    ));
    let definition = instances.add(format!(
        "PRODUCT_DEFINITION('design','',#{formation},#{context})"
    ));
    let shape = instances.add(format!("PRODUCT_DEFINITION_SHAPE('','',#{definition})"));
    instances.add(format!(
        "SHAPE_DEFINITION_REPRESENTATION(#{shape},#{representation})"
    ));
}
