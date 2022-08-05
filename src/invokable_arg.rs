pub extern crate va_list;
pub trait InvokableArg{
    fn get_iarg(args:&mut va_list::VaList)->Self;
}
impl InvokableArg for usize{
    fn get_iarg(args:&mut va_list::VaList)->usize{
       return unsafe{args.get()};
    }
}