using System.Windows;
using WpfApp1.Views;

namespace WpfApp1;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        Title = "Справочники";

        // Attempt to query Rust core library for version and a sample computation.
        // This is wrapped in try/catch so the app still runs if the native DLL is missing.
        try
        {
            var ver = CoreNative.CoreVersion();
            var sum = CoreNative.CoreAdd(2, 3);
            Title = $"Справочники — core v{ver} (2+3={sum})";
        }
        catch (System.DllNotFoundException) { /* Native DLL not present; ignore. */ }
        catch (System.EntryPointNotFoundException) { /* Functions not found; ignore. */ }
        catch (System.BadImageFormatException) { /* Wrong architecture; ignore. */ }
    }

    public void NavigateToOrganizations()
    {
        MainContent.Content = new OrganizationsView();
        Title = "Справочники — Организации";
    }

    public void NavigateToCreateOrganization()
    {
        MainContent.Content = new CreateOrganizationView();
        Title = "Справочники — Создание организации";
    }

    public void NavigateToCreateAddress()
    {
        MainContent.Content = new CreateAddressView();
        Title = "Справочники — Создание адреса";
    }

    public void NavigateToAddresses()
    {
        MainContent.Content = new AddressView();
        Title = "Справочники — Адреса";
    }

    public void NavigateToIndividualEntrepreneurs()
    {
        MainContent.Content = new IndividualEntrepreneursView();
        Title = "Справочники — Индивидуальные предприниматели";
    }

    public void NavigateToCreateIndividualEntrepreneur()
    {
        MainContent.Content = new CreateIndividualEntrepreneurView();
        Title = "Справочники — Создание индивидуального предпринимателя";
    }

    public void NavigateToPhysicalPersons()
    {
        MainContent.Content = new PhysicalPersonsView();
        Title = "Справочники — Физические лица";
    }

    public void NavigateToCreatePhysicalPerson()
    {
        MainContent.Content = new CreatePhysicalPersonView();
        Title = "Справочники — Создание физического лица";
    }

    private void OnAddressesClick(object sender, RoutedEventArgs e)
    {
        NavigateToAddresses();
    }

    private void OnOrganizationsClick(object sender, RoutedEventArgs e)
    {
        NavigateToOrganizations();
    }

    private void OnPhysicalPersonsClick(object sender, RoutedEventArgs e)
    {
        NavigateToPhysicalPersons();
    }

    private void OnIndividualEntrepreneursClick(object sender, RoutedEventArgs e)
    {
        NavigateToIndividualEntrepreneurs();
    }
}
