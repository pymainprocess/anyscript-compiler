#[cfg(target_pointer_width = "32")]
#[allow(unused)]
pub type IntAlias = i32;
#[cfg(target_pointer_width = "64")]
#[allow(unused)]
pub type IntAlias = i64;
#[cfg(target_pointer_width = "32")]
#[allow(unused)]
pub type FloatAlias = f32;
#[cfg(target_pointer_width = "64")]
#[allow(unused)]
pub type FloatAlias = f64;
#[cfg(target_pointer_width = "32")]
#[allow(unused)]
pub type UIntAlias = u32;
#[cfg(target_pointer_width = "64")]
#[allow(unused)]
pub type UIntAlias = u64;