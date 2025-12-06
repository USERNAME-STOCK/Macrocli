using System;
using System.Text;
using System.Windows;

namespace MacropadGUI
{
    public partial class App : Application
    {
        public App()
        {
            // Łap wszystkie wyjątki UI
            this.DispatcherUnhandledException += (sender, e) =>
            {
                ShowFullException("UI ERROR", e.Exception);
                e.Handled = true;
            };

            AppDomain.CurrentDomain.UnhandledException += (sender, e) =>
            {
                ShowFullException("FATAL ERROR", e.ExceptionObject as Exception);
            };
        }

        protected override void OnStartup(StartupEventArgs e)
        {
            try
            {
                base.OnStartup(e);
            }
            catch (Exception ex)
            {
                ShowFullException("STARTUP ERROR", ex);
            }
        }

        private void ShowFullException(string title, Exception? ex)
        {
            if (ex == null)
            {
                MessageBox.Show("Unknown error (null exception)", title);
                return;
            }

            var sb = new StringBuilder();
            sb.AppendLine($"=== {title} ===\n");
            
            int depth = 0;
            Exception? current = ex;
            while (current != null && depth < 10)
            {
                string indent = new string(' ', depth * 2);
                sb.AppendLine($"{indent}[{depth}] {current.GetType().Name}: {current.Message}");
                
                if (!string.IsNullOrEmpty(current.StackTrace))
                {
                    // Pokaż tylko pierwsze 3 linie stack trace
                    var lines = current.StackTrace.Split('\n');
                    for (int i = 0; i < Math.Min(3, lines.Length); i++)
                    {
                        sb.AppendLine($"{indent}    {lines[i].Trim()}");
                    }
                }
                
                current = current.InnerException;
                depth++;
            }

            MessageBox.Show(sb.ToString(), title, MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }
}
