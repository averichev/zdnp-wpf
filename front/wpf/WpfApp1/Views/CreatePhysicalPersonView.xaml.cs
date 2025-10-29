using System;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;

namespace WpfApp1.Views;

public partial class CreatePhysicalPersonView : UserControl
{
    public CreatePhysicalPersonView()
    {
        InitializeComponent();
        Loaded += OnLoaded;
    }

    private void OnLoaded(object? sender, RoutedEventArgs e)
    {
        Loaded -= OnLoaded;
        TryLoadAddresses();
    }

    private void TryLoadAddresses()
    {
        try
        {
            var addresses = CoreNative.CoreListAddresses();
            AddressComboBox.ItemsSource = addresses;
            if (addresses is { Count: > 0 })
            {
                AddressComboBox.SelectedIndex = 0;
            }
        }
        catch (DllNotFoundException ex)
        {
            ShowMessage($"Библиотека zdnp_core не найдена: {ex.Message}", isError: true);
        }
        catch (EntryPointNotFoundException ex)
        {
            ShowMessage($"Не найдены экспортируемые методы в zdnp_core: {ex.Message}", isError: true);
        }
        catch (BadImageFormatException ex)
        {
            ShowMessage($"Не удалось загрузить библиотеку zdnp_core: {ex.Message}", isError: true);
        }
    }

    private void OnClearClick(object sender, RoutedEventArgs e)
    {
        SurnameTextBox.Text = string.Empty;
        NameTextBox.Text = string.Empty;
        PatronymicTextBox.Text = string.Empty;
        SnilsTextBox.Text = string.Empty;
        EmailTextBox.Text = string.Empty;
        AddressComboBox.SelectedIndex = -1;

        StatusTextBlock.Text = string.Empty;
        StatusTextBlock.Visibility = Visibility.Collapsed;
    }

    private void OnSaveClick(object sender, RoutedEventArgs e)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(SurnameTextBox.Text))
            {
                ShowMessage("Поле \"Фамилия\" обязательно.", isError: true);
                return;
            }

            if (string.IsNullOrWhiteSpace(NameTextBox.Text))
            {
                ShowMessage("Поле \"Имя\" обязательно.", isError: true);
                return;
            }

            if (string.IsNullOrWhiteSpace(SnilsTextBox.Text))
            {
                ShowMessage("Поле \"СНИЛС\" обязательно.", isError: true);
                return;
            }

            if (string.IsNullOrWhiteSpace(EmailTextBox.Text))
            {
                ShowMessage("Поле \"Электронная почта\" обязательно.", isError: true);
                return;
            }

            if (AddressComboBox.SelectedValue is not long addressId)
            {
                ShowMessage("Выберите адрес для физического лица.", isError: true);
                return;
            }

            var dto = new CoreNative.PersonDto(
                NameTextBox.Text,
                string.IsNullOrWhiteSpace(PatronymicTextBox.Text) ? null : PatronymicTextBox.Text,
                SurnameTextBox.Text,
                SnilsTextBox.Text,
                EmailTextBox.Text,
                addressId
            );

            if (!CoreNative.CoreCreatePerson(dto, out var personId))
            {
                ShowMessage("Не удалось сохранить физическое лицо в базе данных.", isError: true);
                return;
            }

            var successMessage = $"Физическое лицо сохранено (ID: {personId}).";

            if (Window.GetWindow(this) is MainWindow mainWindow)
            {
                MessageBox.Show(successMessage, "Физическое лицо сохранено", MessageBoxButton.OK, MessageBoxImage.Information);
                mainWindow.NavigateToPhysicalPersons();
                return;
            }

            ShowMessage(successMessage, isError: false);
        }
        catch (DllNotFoundException)
        {
            ShowMessage("Библиотека zdnp_core не найдена. Убедитесь, что она находится рядом с приложением.", isError: true);
        }
        catch (EntryPointNotFoundException)
        {
            ShowMessage("Не удалось найти экспортируемые методы в библиотеке zdnp_core.", isError: true);
        }
        catch (BadImageFormatException)
        {
            ShowMessage("Не удалось загрузить библиотеку zdnp_core (проверьте архитектуру сборки).", isError: true);
        }
    }

    private void ShowMessage(string message, bool isError)
    {
        StatusTextBlock.Text = message;
        StatusTextBlock.Foreground = isError ? Brushes.DarkRed : Brushes.Black;
        StatusTextBlock.Visibility = Visibility.Visible;
    }
}
