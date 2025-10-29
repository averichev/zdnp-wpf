using System;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;

namespace WpfApp1.Views;

public partial class CreateIndividualEntrepreneurView : UserControl
{
    public CreateIndividualEntrepreneurView()
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
        OgrnipTextBox.Text = string.Empty;
        InnTextBox.Text = string.Empty;
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

            if (string.IsNullOrWhiteSpace(OgrnipTextBox.Text))
            {
                ShowMessage("Поле \"ОГРНИП\" обязательно.", isError: true);
                return;
            }

            if (string.IsNullOrWhiteSpace(InnTextBox.Text))
            {
                ShowMessage("Поле \"ИНН\" обязательно.", isError: true);
                return;
            }

            if (AddressComboBox.SelectedValue is not long addressId)
            {
                ShowMessage("Выберите адрес для индивидуального предпринимателя.", isError: true);
                return;
            }

            var dto = new CoreNative.EntrepreneurDto(
                SurnameTextBox.Text,
                NameTextBox.Text,
                string.IsNullOrWhiteSpace(PatronymicTextBox.Text) ? null : PatronymicTextBox.Text,
                OgrnipTextBox.Text,
                InnTextBox.Text,
                addressId,
                string.IsNullOrWhiteSpace(EmailTextBox.Text) ? null : EmailTextBox.Text
            );

            if (!CoreNative.CoreCreateEntrepreneur(dto, out var entrepreneurId))
            {
                ShowMessage("Не удалось сохранить индивидуального предпринимателя в базе данных.", isError: true);
                return;
            }

            var successMessage = $"Индивидуальный предприниматель сохранён (ID: {entrepreneurId}).";

            if (Window.GetWindow(this) is MainWindow mainWindow)
            {
                MessageBox.Show(successMessage, "Предприниматель сохранён", MessageBoxButton.OK, MessageBoxImage.Information);
                mainWindow.NavigateToIndividualEntrepreneurs();
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
