using System.Runtime.CompilerServices;
class Test{
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void ConfoirmConstuctorCall(object self);
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void PassDataArray(int[] data);
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void PassArgCount(int count);
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void SendTestString(string s);
    public static void Main(string[] args){
        string tmp = "|";
        foreach(string arg in args){
            tmp += arg + ",";
        }
        PassArgCount(args.Length);
        PassDataArray(new int[]{0,1,2,3,4,5});
        SendTestString(tmp);
        System.Environment.Exit(0);
    }
    public Test(){
        ConfoirmConstuctorCall(this);
    }
} 
class Secondary{
    public Secondary(int a,int b){
        Test.ConfoirmConstuctorCall(this);
    }
}
