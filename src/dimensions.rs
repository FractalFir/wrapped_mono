pub trait DimensionTrait {
    const DIMENSIONS: usize;
    type Lengths;
    fn zeroed() -> Self::Lengths;
}
macro_rules! def_dim {
    ($name:ident,$count:literal) => {
        pub struct $name;
        impl DimensionTrait for $name {
            type Lengths = [usize; $count];
            const DIMENSIONS: usize = $count;
            fn zeroed() -> Self::Lengths {
                [0; $count]
            }
        }
    };
}
def_dim!(Dim1D, 1);
def_dim!(Dim2D, 2);
def_dim!(Dim3D, 3);
def_dim!(Dim4D, 4);
def_dim!(Dim5D, 5);
def_dim!(Dim6D, 6);
def_dim!(Dim7D, 7);
def_dim!(Dim8D, 8);
def_dim!(Dim9D, 9);
def_dim!(Dim10D, 10);
def_dim!(Dim11D, 11);
def_dim!(Dim12D, 12);
def_dim!(Dim13D, 13);
def_dim!(Dim14D, 14);
def_dim!(Dim15D, 15);
def_dim!(Dim16D, 16);
def_dim!(Dim17D, 17);
def_dim!(Dim18D, 18);
def_dim!(Dim19D, 19);
def_dim!(Dim20D, 20);
