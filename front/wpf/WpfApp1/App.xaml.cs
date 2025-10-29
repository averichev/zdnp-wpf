using System.Windows;

namespace WpfApp1;

/// <summary>
/// Interaction logic for App.xaml
/// </summary>
public partial class App
{
    private static readonly Lock SMigrationLock = new();
    private static bool _sMigrationsApplied;

    protected override void OnStartup(StartupEventArgs e)
    {
        EnsureMigrations();
        base.OnStartup(e);
    }

    private static void EnsureMigrations()
    {
        lock (SMigrationLock)
        {
            if (_sMigrationsApplied)
            {
                return;
            }
        }

        lock (SMigrationLock)
        {
            if (_sMigrationsApplied)
            {
                return;
            }

            if (!CoreNative.EnsureMigrations())
            {
                throw new InvalidOperationException("Failed to run database migrations.");
            }

            _sMigrationsApplied = true;
        }
    }
}