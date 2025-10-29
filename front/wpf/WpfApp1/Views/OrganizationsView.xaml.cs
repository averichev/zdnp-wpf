using System;
using System.Collections.ObjectModel;
using System.Text.Json;
using System.Windows;
using System.Windows.Controls;
using WpfApp1;

namespace WpfApp1.Views;

public partial class OrganizationsView : UserControl
{
    public OrganizationsView()
    {
        InitializeComponent();
        DataContext = this;

        Loaded += OnLoaded;
    }

    private void OnAddOrganizationClick(object sender, RoutedEventArgs e)
    {
        if (Window.GetWindow(this) is MainWindow mainWindow)
        {
            mainWindow.NavigateToCreateOrganization();
        }
    }

    public ObservableCollection<CoreNative.Organization> Organizations { get; } = [];

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        Loaded -= OnLoaded;
        RefreshOrganizations();
    }

    private void RefreshOrganizations()
    {
        try
        {
            var organizations = CoreNative.CoreListOrganizations();

            Organizations.Clear();

            if (organizations is null)
            {
                return;
            }

            foreach (var organization in organizations)
            {
                Organizations.Add(organization);
            }
        }
        catch (DllNotFoundException ex)
        {
            ShowLoadError(ex);
        }
        catch (EntryPointNotFoundException ex)
        {
            ShowLoadError(ex);
        }
        catch (BadImageFormatException ex)
        {
            ShowLoadError(ex);
        }
        catch (JsonException ex)
        {
            ShowLoadError(ex);
        }
    }

    private static void ShowLoadError(Exception ex)
    {
        MessageBox.Show(
            $"Не удалось загрузить список организаций: {ex.Message}",
            "Ошибка",
            MessageBoxButton.OK,
            MessageBoxImage.Error);
    }
}
