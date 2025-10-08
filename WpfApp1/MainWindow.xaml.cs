using System.Windows;
using WpfApp1.Views;

namespace WpfApp1;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        Title = "Справочники";
    }

    private void OnAddressesClick(object sender, RoutedEventArgs e)
    {
        MainContent.Content = new AddressView();
        Title = "Справочники — Адреса";
    }

    private void OnOrganizationsClick(object sender, RoutedEventArgs e)
    {
        MainContent.Content = new OrganizationsView();
        Title = "Справочники — Организации";
    }

    private void OnLegalEntitiesClick(object sender, RoutedEventArgs e)
    {
        MainContent.Content = new LegalEntitiesView();
        Title = "Справочники — Юридические лица";
    }

    private void OnPhysicalPersonsClick(object sender, RoutedEventArgs e)
    {
        MainContent.Content = new PhysicalPersonsView();
        Title = "Справочники — Физические лица";
    }

    private void OnIndividualEntrepreneursClick(object sender, RoutedEventArgs e)
    {
        MainContent.Content = new IndividualEntrepreneursView();
        Title = "Справочники — Индивидуальные предприниматели";
    }
}
