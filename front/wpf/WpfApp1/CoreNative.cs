using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text.Json;

namespace WpfApp1;

public static partial class CoreNative
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

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_create_address")]
    private static extern bool core_create_address(ref AddressDtoFfi dto, out long id);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_list_addresses")]
    private static extern IntPtr core_list_addresses();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_list_organizations")]
    private static extern IntPtr core_list_organizations();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_list_entrepreneurs")]
    private static extern IntPtr core_list_entrepreneurs();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_list_persons")]
    private static extern IntPtr core_list_persons();

    private static readonly JsonSerializerOptions s_jsonOptions = new() { PropertyNameCaseInsensitive = true, PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };

    // Organization interop
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_create_organization")]
    private static extern bool core_create_organization(ref OrganizationDtoFfi dto, out long id);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_create_entrepreneur")]
    private static extern bool core_create_entrepreneur(ref EntrepreneurDtoFfi dto, out long id);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true, EntryPoint = "core_create_person")]
    private static extern bool core_create_person(ref PersonDtoFfi dto, out long id);

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

    public sealed record Address(
        long Id,
        string RegionCode,
        string? Note,
        string? Country,
        string? District,
        string? City,
        string? Settlement,
        string? Street,
        string? Building,
        string? Room
    );

    public sealed record Organization(
        long Id,
        string FullName,
        string AbbreviatedName,
        string? Ogrn,
        string? Rafp,
        string Inn,
        string Kpp,
        long AddressId,
        string Email
    );

    public sealed record class EntrepreneurDto(
        string Surname,
        string Name,
        string? Patronymic,
        string Ogrnip,
        string Inn,
        long AddressId,
        string? Email
    );

    public sealed record Entrepreneur(
        long Id,
        string Surname,
        string Name,
        string? Patronymic,
        string Ogrnip,
        string Inn,
        long AddressId,
        string? Email
    );

    public sealed record class PersonDto(
        string Name,
        string? Patronymic,
        string Surname,
        string Snils,
        string Email,
        long AddressId
    );

    public sealed record Person(
        long Id,
        string Name,
        string? Patronymic,
        string Surname,
        string Snils,
        string Email,
        long AddressId
    );

    public static string? CoreFormatAddress(AddressDto dto)
    {
        var allocations = new List<IntPtr>(9);
        var ffi = CreateAddressDtoFfi(dto, allocations);

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
            FreeAllocations(allocations);
        }
    }

    public static bool CoreCreateAddress(AddressDto dto, out long id)
    {
        var allocations = new List<IntPtr>(9);
        var ffi = CreateAddressDtoFfi(dto, allocations);

        try
        {
            return core_create_address(ref ffi, out id);
        }
        finally
        {
            FreeAllocations(allocations);
        }
    }

    public static IReadOnlyList<Address>? CoreListAddresses()
    {
        var ptr = core_list_addresses();
        if (ptr == IntPtr.Zero)
        {
            return null;
        }

        try
        {
            var json = Marshal.PtrToStringUTF8(ptr);
            if (string.IsNullOrWhiteSpace(json))
            {
                return [];
            }

            var list = JsonSerializer.Deserialize<Address[]>(json, s_jsonOptions);
            return list ?? [];
        }
        finally
        {
            core_free_string(ptr);
        }
    }

    public static IReadOnlyList<Organization>? CoreListOrganizations()
    {
        var ptr = core_list_organizations();
        if (ptr == IntPtr.Zero)
        {
            return null;
        }

        try
        {
            var json = Marshal.PtrToStringUTF8(ptr);
            if (string.IsNullOrWhiteSpace(json))
            {
                return [];
            }

            var list = JsonSerializer.Deserialize<Organization[]>(json, s_jsonOptions);
            return list ?? [];
        }
        finally
        {
            core_free_string(ptr);
        }
    }

    public static IReadOnlyList<Entrepreneur>? CoreListEntrepreneurs()
    {
        var ptr = core_list_entrepreneurs();
        if (ptr == IntPtr.Zero)
        {
            return null;
        }

        try
        {
            var json = Marshal.PtrToStringUTF8(ptr);
            if (string.IsNullOrWhiteSpace(json))
            {
                return [];
            }

            var list = JsonSerializer.Deserialize<Entrepreneur[]>(json, s_jsonOptions);
            return list ?? [];
        }
        finally
        {
            core_free_string(ptr);
        }
    }

    public static IReadOnlyList<Person>? CoreListPersons()
    {
        var ptr = core_list_persons();
        if (ptr == IntPtr.Zero)
        {
            return null;
        }

        try
        {
            var json = Marshal.PtrToStringUTF8(ptr);
            if (string.IsNullOrWhiteSpace(json))
            {
                return [];
            }

            var list = JsonSerializer.Deserialize<Person[]>(json, s_jsonOptions);
            return list ?? [];
        }
        finally
        {
            core_free_string(ptr);
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

    private static AddressDtoFfi CreateAddressDtoFfi(AddressDto dto, List<IntPtr> allocations)
    {
        return new AddressDtoFfi
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
    }

    private static void FreeAllocations(List<IntPtr> allocations)
    {
        foreach (var ptr in allocations)
        {
            if (ptr != IntPtr.Zero)
            {
                Marshal.FreeCoTaskMem(ptr);
            }
        }
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

    // -------- Organization DTO/FFI --------
    public sealed record class OrganizationDto(
        string FullName,
        string AbbreviatedName,
        string? Ogrn,
        string? Rafp,
        string Inn,
        string Kpp,
        long AddressId,
        string Email
    );

    [StructLayout(LayoutKind.Sequential)]
    private struct OrganizationDtoFfi
    {
        public IntPtr FullName;
        public IntPtr AbbreviatedName;
        public IntPtr Ogrn;
        public IntPtr Rafp;
        public IntPtr Inn;
        public IntPtr Kpp;
        public long AddressId;
        public IntPtr Email;
    }

    private static OrganizationDtoFfi CreateOrganizationDtoFfi(OrganizationDto dto, List<IntPtr> allocations)
    {
        return new OrganizationDtoFfi
        {
            FullName = AllocateUtf8(dto.FullName, allocations),
            AbbreviatedName = AllocateUtf8(dto.AbbreviatedName, allocations),
            Ogrn = AllocateUtf8(dto.Ogrn, allocations),
            Rafp = AllocateUtf8(dto.Rafp, allocations),
            Inn = AllocateUtf8(dto.Inn, allocations),
            Kpp = AllocateUtf8(dto.Kpp, allocations),
            AddressId = dto.AddressId,
            Email = AllocateUtf8(dto.Email, allocations)
        };
    }

    public static bool CoreCreateOrganization(OrganizationDto dto, out long id)
    {
        var allocations = new List<IntPtr>(7);
        var ffi = CreateOrganizationDtoFfi(dto, allocations);

        try
        {
            return core_create_organization(ref ffi, out id);
        }
        finally
        {
            FreeAllocations(allocations);
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct EntrepreneurDtoFfi
    {
        public IntPtr Surname;
        public IntPtr Name;
        public IntPtr Patronymic;
        public IntPtr Ogrnip;
        public IntPtr Inn;
        public long AddressId;
        public IntPtr Email;
    }

    private static EntrepreneurDtoFfi CreateEntrepreneurDtoFfi(EntrepreneurDto dto, List<IntPtr> allocations)
    {
        return new EntrepreneurDtoFfi
        {
            Surname = AllocateUtf8(dto.Surname, allocations),
            Name = AllocateUtf8(dto.Name, allocations),
            Patronymic = AllocateUtf8(dto.Patronymic, allocations),
            Ogrnip = AllocateUtf8(dto.Ogrnip, allocations),
            Inn = AllocateUtf8(dto.Inn, allocations),
            AddressId = dto.AddressId,
            Email = AllocateUtf8(dto.Email, allocations)
        };
    }

    public static bool CoreCreateEntrepreneur(EntrepreneurDto dto, out long id)
    {
        var allocations = new List<IntPtr>(6);
        var ffi = CreateEntrepreneurDtoFfi(dto, allocations);

        try
        {
            return core_create_entrepreneur(ref ffi, out id);
        }
        finally
        {
            FreeAllocations(allocations);
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct PersonDtoFfi
    {
        public IntPtr Name;
        public IntPtr Patronymic;
        public IntPtr Surname;
        public IntPtr Snils;
        public IntPtr Email;
        public long AddressId;
    }

    private static PersonDtoFfi CreatePersonDtoFfi(PersonDto dto, List<IntPtr> allocations)
    {
        return new PersonDtoFfi
        {
            Name = AllocateUtf8(dto.Name, allocations),
            Patronymic = AllocateUtf8(dto.Patronymic, allocations),
            Surname = AllocateUtf8(dto.Surname, allocations),
            Snils = AllocateUtf8(dto.Snils, allocations),
            Email = AllocateUtf8(dto.Email, allocations),
            AddressId = dto.AddressId,
        };
    }

    public static bool CoreCreatePerson(PersonDto dto, out long id)
    {
        var allocations = new List<IntPtr>(5);
        var ffi = CreatePersonDtoFfi(dto, allocations);

        try
        {
            return core_create_person(ref ffi, out id);
        }
        finally
        {
            FreeAllocations(allocations);
        }
    }
}
