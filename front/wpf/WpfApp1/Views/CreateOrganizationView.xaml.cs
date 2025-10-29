using System;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;
using WpfApp1;

namespace WpfApp1.Views;

public partial class CreateOrganizationView : UserControl
{
    public CreateOrganizationView()
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
        FullNameTextBox.Text = string.Empty;
        AbbreviatedNameTextBox.Text = string.Empty;
        OgrnTextBox.Text = string.Empty;
        RafpTextBox.Text = string.Empty;
        InnTextBox.Text = string.Empty;
        KppTextBox.Text = string.Empty;
        EmailTextBox.Text = string.Empty;
        AddressComboBox.SelectedIndex = -1;

        StatusTextBlock.Text = string.Empty;
        StatusTextBlock.Visibility = Visibility.Collapsed;
    }

    private void OnSaveClick(object sender, RoutedEventArgs e)
    {
        try
        {
            // Validate required fields
            if (string.IsNullOrWhiteSpace(FullNameTextBox.Text))
            {
                ShowMessage("Поле \"Полное наименование\" обязательно.", isError: true);
                return;
            }
            if (string.IsNullOrWhiteSpace(AbbreviatedNameTextBox.Text))
            {
                ShowMessage("Поле \"Сокращённое наименование\" обязательно.", isError: true);
                return;
            }
            if (string.IsNullOrWhiteSpace(InnTextBox.Text))
            {
                ShowMessage("Поле \"ИНН\" обязательно.", isError: true);
                return;
            }
            if (string.IsNullOrWhiteSpace(KppTextBox.Text))
            {
                ShowMessage("Поле \"КПП\" обязательно.", isError: true);
                return;
            }
            if (string.IsNullOrWhiteSpace(EmailTextBox.Text))
            {
                ShowMessage("Поле \"Электронная почта\" обязательно.", isError: true);
                return;
            }
            if (AddressComboBox.SelectedValue is not long addressId)
            {
                ShowMessage("Выберите адрес для организации.", isError: true);
                return;
            }

            var dto = new CoreNative.OrganizationDto(
                FullNameTextBox.Text,
                AbbreviatedNameTextBox.Text,
                string.IsNullOrWhiteSpace(OgrnTextBox.Text) ? null : OgrnTextBox.Text,
                string.IsNullOrWhiteSpace(RafpTextBox.Text) ? null : RafpTextBox.Text,
                InnTextBox.Text,
                KppTextBox.Text,
                addressId,
                EmailTextBox.Text
            );

            if (!CoreNative.CoreCreateOrganization(dto, out var organizationId))
            {
                ShowMessage("Не удалось сохранить организацию в базе данных.", isError: true);
                return;
            }

            var successMessage = $"Организация сохранена (ID: {organizationId}).";

            if (Window.GetWindow(this) is MainWindow mainWindow)
            {
                MessageBox.Show(successMessage, "Организация сохранена", MessageBoxButton.OK, MessageBoxImage.Information);
                mainWindow.NavigateToOrganizations();
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
