using System.Runtime.CompilerServices;
class Test{
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void PassArgCount(int );
    public static void Main(string[] args){
        int tmp = args.Length;
        PassArgCount(tmp);
        System.Environment.Exit(0);
    }
} 
