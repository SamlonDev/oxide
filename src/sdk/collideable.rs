use crate::math::{angles::Angles, vector3::Vector3};

use super::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTCollideable {
    _pad: [usize; 1],
    pub obb_mins_pre_scaled: cfn!(&Vector3, &Collideable),
    pub obb_maxs_pre_scaled: cfn!(&Vector3, &Collideable),
    pub obb_mins: cfn!(&Vector3, &Collideable),
    pub obb_maxs: cfn!(&Vector3, &Collideable),
    _pad1: [usize; 5],
	pub get_origin: cfn!(&Vector3, &Collideable),
	pub get_angles: cfn!(&Angles, &Collideable),
}

pub type Collideable = WithVmt<VMTCollideable>;
