using System;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;
using WpfApp1;

namespace WpfApp1.Views;

public partial class CreateAddressView : UserControl
{
    public CreateAddressView()
    {
        InitializeComponent();
    }

    private void OnSaveClick(object sender, RoutedEventArgs e)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(RegionCodeTextBox.Text))
            {
                ShowMessage("Поле \"Код региона\" обязательно для заполнения.", isError: true);
                return;
            }

            var dto = new CoreNative.AddressDto(
                RegionCodeTextBox.Text,
                NoteTextBox.Text,
                CountryTextBox.Text,
                DistrictTextBox.Text,
                CityTextBox.Text,
                SettlementTextBox.Text,
                StreetTextBox.Text,
                BuildingTextBox.Text,
                RoomTextBox.Text
            );

            var formatted = CoreNative.CoreFormatAddress(dto);

            if (string.IsNullOrWhiteSpace(formatted))
            {
                ShowMessage("Не удалось отформатировать адрес.", isError: true);
            }
            else
            {
                ShowMessage(formatted!, isError: false);
            }
        }
        catch (DllNotFoundException)
        {
            ShowMessage("Библиотека zdnp_core не найдена. Убедитесь, что она находится рядом с приложением.", isError: true);
        }
        catch (EntryPointNotFoundException)
        {
            ShowMessage("Метод core_format_address отсутствует в библиотеке zdnp_core.", isError: true);
        }
        catch (BadImageFormatException)
        {
            ShowMessage("Не удалось загрузить библиотеку zdnp_core (проверьте архитектуру сборки).", isError: true);
        }
    }

    private void OnClearClick(object sender, RoutedEventArgs e)
    {
        RegionCodeTextBox.Text = string.Empty;
        NoteTextBox.Text = string.Empty;
        CountryTextBox.Text = string.Empty;
        DistrictTextBox.Text = string.Empty;
        CityTextBox.Text = string.Empty;
        SettlementTextBox.Text = string.Empty;
        StreetTextBox.Text = string.Empty;
        BuildingTextBox.Text = string.Empty;
        RoomTextBox.Text = string.Empty;

        FormattedAddressTextBlock.Text = string.Empty;
        FormattedAddressTextBlock.Visibility = Visibility.Collapsed;
    }

    private void ShowMessage(string message, bool isError)
    {
        FormattedAddressTextBlock.Text = message;
        FormattedAddressTextBlock.Foreground = isError ? Brushes.DarkRed : Brushes.Black;
        FormattedAddressTextBlock.Visibility = Visibility.Visible;
    }
}
