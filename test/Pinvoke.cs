using System.Runtime.CompilerServices;
class Test{
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void PassArgCount(int count);
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void SendTestString(string s);
    public static void Main(string[] args){
        int tmp = args.Length;
        PassArgCount(tmp);
        SendTestString("Hello From Mono!");
        System.Environment.Exit(0);
    }
} 
