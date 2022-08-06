pub trait InvokableArg{
    type SourceType;
    fn get_iarg(arg:Self::SourceType)->Self;
}
impl InvokableArg for usize{
    type SourceType = usize;
    fn get_iarg(arg:Self::SourceType)->usize{
       return arg;
    }
}
impl InvokableArg for i32{
    type SourceType = i32;
    fn get_iarg(arg:Self::SourceType)->i32{
       return arg;
    }
}