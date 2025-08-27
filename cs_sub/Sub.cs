using System;
using System.Runtime.InteropServices;

public class SubLib {
    [UnmanagedCallersOnly(EntryPoint="cs_sub")]
    public static int Sub(int a, int b) {
        return a - b;
    }
}
