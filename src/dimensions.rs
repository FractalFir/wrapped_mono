
pub trait DimensionTrait{
    const DIMENSIONS:usize; 
    type Lengths;
    fn zeroed()->Self::Lengths;
}
macro_rules! def_dim{
    ($name:ident,$count:literal)=>{
        pub struct $name;
        impl DimensionTrait for $name{
            type Lengths = [usize;$count];
            const DIMENSIONS:usize = $count;
               fn zeroed()->Self::Lengths{
            [0;$count]
            }
        }
    }
}
def_dim!(Dim1D,1);
def_dim!(Dim2D,2);
def_dim!(Dim3D,3);
def_dim!(Dim4D,4);
def_dim!(Dim5D,5);
