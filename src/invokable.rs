extern crate va_list;
trait InvocableArg{
    fn get_iarg(args:&mut va_list::VaList)->Self;
}
impl InvocableArg for usize{
    fn get_iarg(args:&mut va_list::VaList)->usize{
       return unsafe{args.get()};
    }
}