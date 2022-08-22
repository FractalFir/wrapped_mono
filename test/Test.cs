interface IInterfaceOne{
    void SomeInterfaceFunction();
}
class TestFunctions : IInterfaceOne{
    public void SomeInterfaceFunction(){

    }
    //function used to test if basic static function call works
    public static int GetOne(){
        return 1;
    }
    //function used to test if basic function call works
    public static int GetTwo(){
        return 2;
    }
    //function returning null.
    public static int? GetObject(){
        return null;
    }
    //function used to test if argument passing works
    public static int GetArg(int arg){
        return arg;
    }
} 
static class TestMainEntry{
    public static void Main(string[] ars){
        return;
    }
}