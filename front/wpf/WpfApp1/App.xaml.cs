using System;
using System.Configuration;
using System.Data;
using System.Windows;

namespace WpfApp1;

/// <summary>
/// Interaction logic for App.xaml
/// </summary>
public partial class App : Application
{
    private static readonly object s_migrationLock = new();
    private static bool s_migrationsApplied;

    protected override void OnStartup(StartupEventArgs e)
    {
        EnsureMigrations();
        base.OnStartup(e);
    }

    private static void EnsureMigrations()
    {
        if (s_migrationsApplied)
        {
            return;
        }

        lock (s_migrationLock)
        {
            if (s_migrationsApplied)
            {
                return;
            }

            if (!CoreNative.EnsureMigrations())
            {
                throw new InvalidOperationException("Failed to run database migrations.");
            }

            s_migrationsApplied = true;
        }
    }
}
