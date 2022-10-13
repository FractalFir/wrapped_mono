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
    public static string PrintTypes(System.Type[] types){
    	string s = "Types:\"";
    	foreach(System.Type type in types){
            if(type != null){
                s += type + ",";
            }
    		
    	}
    	s += "\"";
    	return s;
    }
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
    public static long Mul(long arg,long arg2){
        return arg * arg2;
    }
    public static long Mul(long arg,long arg2,long arg3){
        return arg * arg2 * arg3;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4){
        return arg * arg2 * arg3 * arg4;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5){
        return arg * arg2 * arg3 * arg4 * arg5;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8,long arg9){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8 * arg9;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8,long arg9,long arg10){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8 * arg9 * arg10;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8,long arg9,long arg10,long arg11){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8 * arg9 * arg10 * arg11;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8,long arg9,long arg10,long arg11,long arg12){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8 * arg9 * arg10 * arg11 * arg12;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8,long arg9,long arg10,long arg11,long arg12,long arg13){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8 * arg9 * arg10 * arg11 * arg12 * arg13;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8,long arg9,long arg10,long arg11,long arg12,long arg13,long arg14){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8 * arg9 * arg10 * arg11 * arg12 * arg13 * arg14;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8,long arg9,long arg10,long arg11,long arg12,long arg13,long arg14,long arg15){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8 * arg9 * arg10 * arg11 * arg12 * arg13 * arg14 * arg15;
    }
    public static long Mul(long arg,long arg2,long arg3,long arg4,long arg5,long arg6,long arg7,long arg8,long arg9,long arg10,long arg11,long arg12,long arg13,long arg14,long arg15,long arg16){
        return arg * arg2 * arg3 * arg4 * arg5 * arg6 * arg7 * arg8 * arg9 * arg10 * arg11 * arg12 * arg13 * arg14 * arg15 * arg16;
    }
    public static int StrTest(string a,string b,string c,string d){
        string res = a + b + c + d;
        return res.Length;
    }
    static int DelFNC(int x,int y){
        return x % y + x;
    }
    public delegate int TestDelegate(int x,int y);
    public static TestDelegate GetDelegate(){
        return new TestDelegate(DelFNC);
    }
    public static void ExceptionThrower(){
        throw new System.InvalidOperationException("Logfile cannot be read-only");
    }
} 
static class TestMainEntry{
    public static void Main(string[] ars){
        return;
    }
}
