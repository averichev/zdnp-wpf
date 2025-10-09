using System;
using System.Runtime.InteropServices;

namespace WpfApp1;

internal static partial class CoreNative
{
    private const string DllName = "zdnp_core"; // resolves to zdnp_core.dll on Windows

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_add")]
    public static extern int CoreAdd(int a, int b);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_version")]
    private static extern IntPtr core_version();

    public static string CoreVersion()
    {
        var ptr = core_version();
        // The string is ASCII/UTF-8 and points to a static buffer in Rust; do not free.
        return Marshal.PtrToStringUTF8(ptr) ?? string.Empty;
    }
}
