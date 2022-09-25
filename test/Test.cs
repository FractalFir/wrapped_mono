interface IInterfaceOne{
    void SomeInterfaceFunction();
}
enum CLikeEnum{
    Val = 1,
    Val2 = 2,
    Val3 = 612,
}
class CtorTestClassParrent{
    protected CtorTestClassParrent(){}
    public CtorTestClassParrent(int a,int b){}
}
class CtorTestClass: CtorTestClassParrent{
    public CtorTestClass(int a,string b){}
    public CtorTestClass(object a,string b){}
}
class TestFunctions : IInterfaceOne{
    public int someField = 5;
    public static int[,] Get2DIntArray(){
        return new int[8,16];
    }
    public int GetSomeFiled(){
        return this.someField;
    }
    public void SomeInterfaceFunction(){

    }
    public static CLikeEnum GetEnum(CLikeEnum input){
        return input;
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
    public static int Mul(int arg,int arg2){
        return arg * arg2;
    }
    public static int Mul(int arg,int arg2,int arg3){
        return arg * arg2 * arg3;
    }
    public static int Mul(int arg,int arg2,int arg3,int arg4){
        return arg * arg2 * arg3 * arg4;
    }
} 
static class TestMainEntry{
    public static void Main(string[] ars){
        return;
    }
}