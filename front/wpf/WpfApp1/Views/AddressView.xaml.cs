using System;
using System.Collections.ObjectModel;
using System.Text.Json;
using System.Windows;
using System.Windows.Controls;
using WpfApp1;

namespace WpfApp1.Views;

public partial class AddressView : UserControl
{
    public AddressView()
    {
        InitializeComponent();
        DataContext = this;

        Loaded += OnLoaded;
    }

    private void OnAddAddressClick(object sender, RoutedEventArgs e)
    {
        if (Window.GetWindow(this) is MainWindow mainWindow)
        {
            mainWindow.NavigateToCreateAddress();
        }
    }

    public ObservableCollection<CoreNative.Address> Addresses { get; } = [];

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        Loaded -= OnLoaded;
        RefreshAddresses();
    }

    private void RefreshAddresses()
    {
        try
        {
            var addresses = CoreNative.CoreListAddresses();

            Addresses.Clear();

            if (addresses is null)
            {
                return;
            }

            foreach (var address in addresses)
            {
                Addresses.Add(address);
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
            $"Не удалось загрузить список адресов: {ex.Message}",
            "Ошибка",
            MessageBoxButton.OK,
            MessageBoxImage.Error);
    }
}
