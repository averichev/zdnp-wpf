using System.Windows;
using System.Windows.Controls;
using WpfApp1;

namespace WpfApp1.Views;

public partial class AddressView : UserControl
{
    public AddressView()
    {
        InitializeComponent();
    }

    private void OnAddAddressClick(object sender, RoutedEventArgs e)
    {
        if (Window.GetWindow(this) is MainWindow mainWindow)
        {
            mainWindow.NavigateToCreateAddress();
        }
    }
}
