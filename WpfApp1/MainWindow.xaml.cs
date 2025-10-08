using System.Windows;
using WpfApp1.Views;

namespace WpfApp1;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        NavigateToFile();
    }

    private void OnFileMenuClick(object sender, RoutedEventArgs e)
    {
        NavigateToFile();
    }

    private void OnEditMenuClick(object sender, RoutedEventArgs e)
    {
        MainContent.Content = new EditView();
        Title = "Правка";
    }

    private void OnViewMenuClick(object sender, RoutedEventArgs e)
    {
        MainContent.Content = new ViewView();
        Title = "Вид";
    }

    private void OnNewAddressClick(object sender, RoutedEventArgs e)
    {
        MainContent.Content = new AddressView();
        Title = "Адрес";
    }

    private void NavigateToFile()
    {
        MainContent.Content = new FileView();
        Title = "Файл";
    }
}
