using System.Runtime.CompilerServices;
class Test{
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void ConfoirmConstuctorCall(object self);
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void PassDataArray(int[] data);
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void PassArgCount(int count);
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern int SendTestString(string s);
    ///Gets a null object
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern object GetObject();
    public static void Main(string[] args){
        string tmp = "|";
        foreach(string arg in args){
            tmp += arg + ",";
        }
        PassArgCount(args.Length);
        PassDataArray(new int[]{0,1,2,3,4,5});
        int STRes = SendTestString(tmp);
        if(STRes!= 5){
            throw new System.Exception($"Recived wrong value!:{STRes}");
        }
        object obj = GetObject();
        if(obj != null){
            throw new System.Exception($"Recived something else than null!");
        }
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
