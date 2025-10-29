using System;
using System.Collections.ObjectModel;
using System.Text.Json;
using System.Windows;
using System.Windows.Controls;

namespace WpfApp1.Views;

public partial class PhysicalPersonsView : UserControl
{
    public PhysicalPersonsView()
    {
        InitializeComponent();
        DataContext = this;

        Loaded += OnLoaded;
    }

    public ObservableCollection<CoreNative.Person> Persons { get; } = [];

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        Loaded -= OnLoaded;
        RefreshPersons();
    }

    private void OnAddPersonClick(object sender, RoutedEventArgs e)
    {
        if (Window.GetWindow(this) is MainWindow mainWindow)
        {
            mainWindow.NavigateToCreatePhysicalPerson();
        }
    }

    private void RefreshPersons()
    {
        try
        {
            var persons = CoreNative.CoreListPersons();

            Persons.Clear();

            if (persons is null)
            {
                return;
            }

            foreach (var person in persons)
            {
                Persons.Add(person);
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
            $"Не удалось загрузить список физических лиц: {ex.Message}",
            "Ошибка",
            MessageBoxButton.OK,
            MessageBoxImage.Error);
    }
}
