using System;
using System.Collections.ObjectModel;
using System.Text.Json;
using System.Windows;
using System.Windows.Controls;

namespace WpfApp1.Views;

public partial class IndividualEntrepreneursView : UserControl
{
    public IndividualEntrepreneursView()
    {
        InitializeComponent();
        DataContext = this;

        Loaded += OnLoaded;
    }

    public ObservableCollection<CoreNative.Entrepreneur> Entrepreneurs { get; } = [];

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        Loaded -= OnLoaded;
        RefreshEntrepreneurs();
    }

    private void OnAddEntrepreneurClick(object sender, RoutedEventArgs e)
    {
        if (Window.GetWindow(this) is MainWindow mainWindow)
        {
            mainWindow.NavigateToCreateIndividualEntrepreneur();
        }
    }

    private void RefreshEntrepreneurs()
    {
        try
        {
            var entrepreneurs = CoreNative.CoreListEntrepreneurs();

            Entrepreneurs.Clear();

            if (entrepreneurs is null)
            {
                return;
            }

            foreach (var entrepreneur in entrepreneurs)
            {
                Entrepreneurs.Add(entrepreneur);
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
            $"Не удалось загрузить список индивидуальных предпринимателей: {ex.Message}",
            "Ошибка",
            MessageBoxButton.OK,
            MessageBoxImage.Error);
    }
}
