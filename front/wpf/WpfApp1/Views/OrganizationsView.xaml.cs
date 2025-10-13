using System.Windows;
using System.Windows.Controls;
using WpfApp1;

namespace WpfApp1.Views;

public partial class OrganizationsView : UserControl
{
    public OrganizationsView()
    {
        InitializeComponent();
    }

    private void OnAddOrganizationClick(object sender, RoutedEventArgs e)
    {
        if (Window.GetWindow(this) is MainWindow mainWindow)
        {
            mainWindow.NavigateToCreateOrganization();
        }
    }
}
