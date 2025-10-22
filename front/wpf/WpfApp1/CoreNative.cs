using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace WpfApp1;

internal static partial class CoreNative
{
    private const string DllName = "zdnp_core"; // resolves to zdnp_core.dll on Windows

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_add")]
    public static extern int CoreAdd(int a, int b);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_version")]
    private static extern IntPtr core_version();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "migrations_run")]
    private static extern bool migrations_run();

    private static readonly Lazy<bool> s_migrations = new(() => migrations_run());

    public static bool EnsureMigrations() => s_migrations.Value;

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_format_address")]
    private static extern IntPtr core_format_address(ref AddressDtoFfi dto);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_free_string")]
    private static extern void core_free_string(IntPtr ptr);

    public static string CoreVersion()
    {
        var ptr = core_version();
        // The string is ASCII/UTF-8 and points to a static buffer in Rust; do not free.
        return Marshal.PtrToStringUTF8(ptr) ?? string.Empty;
    }

    public sealed record class AddressDto(
        string? RegionCode,
        string? Note,
        string? Country,
        string? District,
        string? City,
        string? Settlement,
        string? Street,
        string? Building,
        string? Room
    );

    public static string? CoreFormatAddress(AddressDto dto)
    {
        var allocations = new List<IntPtr>(9);
        var ffi = new AddressDtoFfi
        {
            RegionCode = AllocateUtf8(dto.RegionCode, allocations),
            Note = AllocateUtf8(dto.Note, allocations),
            Country = AllocateUtf8(dto.Country, allocations),
            District = AllocateUtf8(dto.District, allocations),
            City = AllocateUtf8(dto.City, allocations),
            Settlement = AllocateUtf8(dto.Settlement, allocations),
            Street = AllocateUtf8(dto.Street, allocations),
            Building = AllocateUtf8(dto.Building, allocations),
            Room = AllocateUtf8(dto.Room, allocations),
        };

        try
        {
            var resultPtr = core_format_address(ref ffi);
            if (resultPtr == IntPtr.Zero)
            {
                return null;
            }

            try
            {
                return Marshal.PtrToStringUTF8(resultPtr);
            }
            finally
            {
                core_free_string(resultPtr);
            }
        }
        finally
        {
            foreach (var ptr in allocations)
            {
                if (ptr != IntPtr.Zero)
                {
                    Marshal.FreeCoTaskMem(ptr);
                }
            }
        }
    }

    private static IntPtr AllocateUtf8(string? value, List<IntPtr> allocations)
    {
        if (string.IsNullOrWhiteSpace(value))
        {
            return IntPtr.Zero;
        }

        var ptr = Marshal.StringToCoTaskMemUTF8(value);
        allocations.Add(ptr);
        return ptr;
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct AddressDtoFfi
    {
        public IntPtr RegionCode;
        public IntPtr Note;
        public IntPtr Country;
        public IntPtr District;
        public IntPtr City;
        public IntPtr Settlement;
        public IntPtr Street;
        public IntPtr Building;
        public IntPtr Room;
    }
}
