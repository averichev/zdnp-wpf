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
        string? PostalCode,
        string? Country,
        string? Region,
        string? District,
        string? City,
        string? Locality,
        string? Street,
        string? House,
        string? Building,
        string? Structure,
        string? Apartment,
        string? Comment
    );

    public static string? CoreFormatAddress(AddressDto dto)
    {
        var allocations = new List<IntPtr>(12);
        var ffi = new AddressDtoFfi
        {
            PostalCode = AllocateUtf8(dto.PostalCode, allocations),
            Country = AllocateUtf8(dto.Country, allocations),
            Region = AllocateUtf8(dto.Region, allocations),
            District = AllocateUtf8(dto.District, allocations),
            City = AllocateUtf8(dto.City, allocations),
            Locality = AllocateUtf8(dto.Locality, allocations),
            Street = AllocateUtf8(dto.Street, allocations),
            House = AllocateUtf8(dto.House, allocations),
            Building = AllocateUtf8(dto.Building, allocations),
            Structure = AllocateUtf8(dto.Structure, allocations),
            Apartment = AllocateUtf8(dto.Apartment, allocations),
            Comment = AllocateUtf8(dto.Comment, allocations),
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
        public IntPtr PostalCode;
        public IntPtr Country;
        public IntPtr Region;
        public IntPtr District;
        public IntPtr City;
        public IntPtr Locality;
        public IntPtr Street;
        public IntPtr House;
        public IntPtr Building;
        public IntPtr Structure;
        public IntPtr Apartment;
        public IntPtr Comment;
    }
}
