using System.Windows;
using System.Windows.Input;

namespace MacropadGUI
{
    public partial class EditLabelDialog : Window
    {
        public string NewLabel { get; private set; } = "";
        public new string Title 
        { 
            get => TitleText.Text.Replace("Edit ", "");
            set => TitleText.Text = $"Edit {value}";
        }

        public EditLabelDialog(int buttonNumber, string currentLabel, string mapping)
        {
            InitializeComponent();
            
            TitleText.Text = buttonNumber > 0 ? $"Edit Button {buttonNumber}" : "Edit";
            MappingText.Text = string.IsNullOrEmpty(mapping) ? "(not configured)" : mapping;
            LabelInput.Text = currentLabel;
            
            // Focus on input
            Loaded += (s, e) => 
            {
                LabelInput.Focus();
                LabelInput.SelectAll();
            };
            
            // Enter to save
            LabelInput.KeyDown += (s, e) =>
            {
                if (e.Key == Key.Enter) Save_Click(null!, null!);
                if (e.Key == Key.Escape) Cancel_Click(null!, null!);
            };
        }

        private void Border_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
        {
            if (e.ButtonState == MouseButtonState.Pressed)
                DragMove();
        }

        private void Save_Click(object sender, RoutedEventArgs e)
        {
            NewLabel = LabelInput.Text.Trim();
            DialogResult = true;
            Close();
        }

        private void Cancel_Click(object sender, RoutedEventArgs e)
        {
            DialogResult = false;
            Close();
        }
    }
}
