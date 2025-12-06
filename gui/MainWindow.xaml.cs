using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Controls.Primitives;
using System.Windows.Media;
using Microsoft.Win32;

namespace MacropadGUI
{
    public partial class MainWindow : Window
    {
        private string MacrocliPath = GetMacrocliPath();
        
        private static string GetMacrocliPath()
        {
            var exeDir = AppDomain.CurrentDomain.BaseDirectory;
            var portablePath = Path.Combine(exeDir, "macrocli.exe");
            if (File.Exists(portablePath)) return portablePath;
            return @"D:\Macrocli\target\debug\macrocli.exe";
        }

        private MacropadConfig _config = new();
        private int _currentLayer = 0;
        private (int row, int col)? _selectedButton = null;
        private (int knob, string action)? _selectedKnob = null;
        private Button[,]? _buttonGrid;
        private List<KnobUI> _knobUIs = new();
        private OsdWindow? _osdWindow;

        // For storing button content references
        private record KnobUI(Border Container, Button CCW, Button Press, Button CW);

        public MainWindow()
        {
            InitializeComponent();
            InitializeDefaultConfig(4, 4, 3);
            BuildDynamicUI();
            RefreshUI();

            if (!File.Exists(MacrocliPath))
            {
                Log($"⚠️ macrocli.exe not found at {MacrocliPath}");
            }
        }

        private void InitializeDefaultConfig(int rows, int cols, int knobs)
        {
            _config = new MacropadConfig
            {
                Rows = rows,
                Cols = cols,
                Knobs = knobs,
                Layers = new List<LayerConfig>
                {
                    CreateEmptyLayer(rows, cols, knobs),
                    CreateEmptyLayer(rows, cols, knobs),
                    CreateEmptyLayer(rows, cols, knobs)
                }
            };
        }

        private LayerConfig CreateEmptyLayer(int rows, int cols, int knobs)
        {
            var layer = new LayerConfig
            {
                Buttons = new ButtonConfig[rows, cols],
                Knobs = new KnobConfig[knobs]
            };

            for (int r = 0; r < rows; r++)
                for (int c = 0; c < cols; c++)
                    layer.Buttons[r, c] = new ButtonConfig();

            for (int k = 0; k < knobs; k++)
                layer.Knobs[k] = new KnobConfig();

            return layer;
        }

        private void BuildDynamicUI()
        {
            int rows = _config.Rows;
            int cols = _config.Cols;
            int knobs = _config.Knobs;

            // Update header info
            DeviceInfo.Text = $"Device: {rows}×{cols} buttons, {knobs} knobs";
            ButtonsCount.Text = $" ({rows * cols})";

            // Build button grid
            ButtonGrid.Rows = rows;
            ButtonGrid.Columns = cols;
            ButtonGrid.Children.Clear();
            _buttonGrid = new Button[rows, cols];

            for (int row = 0; row < rows; row++)
            {
                for (int col = 0; col < cols; col++)
                {
                    var btn = CreateKeyButton(row, col);
                    _buttonGrid[row, col] = btn;
                    ButtonGrid.Children.Add(btn);
                }
            }

            // Build knobs
            BuildKnobsUI(knobs);
        }

        private Button CreateKeyButton(int row, int col)
        {
            var keyNum = row * _config.Cols + col + 1;
            
            var btn = new Button
            {
                Style = (Style)FindResource("KeyButton"),
                Tag = (row, col),
                ToolTip = $"Button {keyNum}"
            };
            btn.Click += GridButton_Click;
            
            // Set initial content
            UpdateButtonContent(btn, keyNum, "", "");
            
            return btn;
        }

        private void UpdateButtonContent(Button btn, int keyNum, string label, string mapping)
        {
            var stack = new StackPanel { VerticalAlignment = VerticalAlignment.Center };
            
            // Key number (small, top-right feel)
            stack.Children.Add(new TextBlock
            {
                Text = $"[{keyNum:D2}]",
                FontSize = 9,
                Foreground = (Brush)FindResource("TextMutedBrush"),
                HorizontalAlignment = HorizontalAlignment.Right,
                Margin = new Thickness(0, 0, 0, 4)
            });
            
            // Label (big, bold) or placeholder
            var displayLabel = string.IsNullOrEmpty(label) ? "---" : label;
            stack.Children.Add(new TextBlock
            {
                Text = displayLabel,
                FontSize = 14,
                FontWeight = FontWeights.SemiBold,
                Foreground = string.IsNullOrEmpty(label) 
                    ? (Brush)FindResource("TextMutedBrush") 
                    : (Brush)FindResource("TextBrush"),
                TextAlignment = TextAlignment.Center,
                TextWrapping = TextWrapping.Wrap,
                TextTrimming = TextTrimming.CharacterEllipsis,
                MaxHeight = 40
            });
            
            // Mapping (small, gray, monospace)
            if (!string.IsNullOrEmpty(mapping))
            {
                var shortMapping = mapping.Length > 18 ? mapping.Substring(0, 15) + "..." : mapping;
                stack.Children.Add(new TextBlock
                {
                    Text = shortMapping,
                    FontSize = 9,
                    FontFamily = new FontFamily("Consolas"),
                    Foreground = (Brush)FindResource("TextMutedBrush"),
                    TextAlignment = TextAlignment.Center,
                    Margin = new Thickness(0, 4, 0, 0)
                });
            }
            
            btn.Content = stack;
        }

        private void BuildKnobsUI(int knobs)
        {
            KnobsContainer.Children.Clear();
            _knobUIs.Clear();

            if (knobs == 0)
            {
                KnobsPanel.Visibility = Visibility.Collapsed;
                KnobsColumn.Width = new GridLength(0);
                return;
            }

            KnobsPanel.Visibility = Visibility.Visible;
            KnobsColumn.Width = new GridLength(200);
            KnobsLabel.Text = $"KNOBS ({knobs})";

            for (int k = 0; k < knobs; k++)
            {
                var knobUI = CreateKnobUI(k);
                KnobsContainer.Children.Add(knobUI.Container);
                _knobUIs.Add(knobUI);
            }
        }

        private KnobUI CreateKnobUI(int knobIndex)
        {
            // Container border
            var border = new Border
            {
                Background = (Brush)FindResource("BgCardBrush"),
                CornerRadius = new CornerRadius(10),
                Padding = new Thickness(12),
                Margin = new Thickness(0, 0, 0, 10)
            };
            border.Effect = new System.Windows.Media.Effects.DropShadowEffect
            {
                BlurRadius = 6,
                ShadowDepth = 1,
                Opacity = 0.2
            };

            var stack = new StackPanel();
            
            // Knob header with icon
            var header = new StackPanel { Orientation = Orientation.Horizontal, Margin = new Thickness(0, 0, 0, 10) };
            header.Children.Add(new TextBlock
            {
                Text = "◉",
                FontSize = 18,
                Foreground = (Brush)FindResource("AccentBrush"),
                Margin = new Thickness(0, 0, 8, 0)
            });
            header.Children.Add(new TextBlock
            {
                Text = $"Knob {knobIndex + 1}",
                FontSize = 14,
                FontWeight = FontWeights.Bold,
                Foreground = (Brush)FindResource("TextBrush"),
                VerticalAlignment = VerticalAlignment.Center
            });
            stack.Children.Add(header);

            // Three action buttons (vertical)
            var ccwBtn = CreateKnobActionButton(knobIndex, "ccw", "↺ CCW", "Counter-clockwise");
            var pressBtn = CreateKnobActionButton(knobIndex, "press", "⏺ Press", "Button press");
            var cwBtn = CreateKnobActionButton(knobIndex, "cw", "↻ CW", "Clockwise");

            stack.Children.Add(ccwBtn);
            stack.Children.Add(pressBtn);
            stack.Children.Add(cwBtn);

            border.Child = stack;
            return new KnobUI(border, ccwBtn, pressBtn, cwBtn);
        }

        private Button CreateKnobActionButton(int knobIndex, string action, string text, string tooltip)
        {
            var btn = new Button
            {
                Style = (Style)FindResource("KnobButton"),
                Tag = $"{knobIndex},{action}",
                ToolTip = tooltip,
                HorizontalContentAlignment = HorizontalAlignment.Left,
                Padding = new Thickness(10, 8, 10, 8),
                Margin = new Thickness(0, 2, 0, 2)
            };
            btn.Click += KnobButton_Click;
            
            UpdateKnobButtonContent(btn, text, "", "");
            return btn;
        }

        private void UpdateKnobButtonContent(Button btn, string prefix, string label, string mapping)
        {
            var grid = new Grid();
            grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(50) });
            grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });

            // Prefix (CCW/Press/CW)
            var prefixBlock = new TextBlock
            {
                Text = prefix,
                FontSize = 11,
                Foreground = (Brush)FindResource("AccentBrush"),
                VerticalAlignment = VerticalAlignment.Center
            };
            Grid.SetColumn(prefixBlock, 0);
            grid.Children.Add(prefixBlock);

            // Label or mapping
            var display = !string.IsNullOrEmpty(label) ? label : 
                         (!string.IsNullOrEmpty(mapping) ? mapping : "---");
            if (display.Length > 12) display = display.Substring(0, 10) + "...";
            
            var valueBlock = new TextBlock
            {
                Text = display,
                FontSize = 10,
                Foreground = string.IsNullOrEmpty(label) && string.IsNullOrEmpty(mapping)
                    ? (Brush)FindResource("TextMutedBrush")
                    : (Brush)FindResource("TextSecondaryBrush"),
                VerticalAlignment = VerticalAlignment.Center,
                TextTrimming = TextTrimming.CharacterEllipsis
            };
            Grid.SetColumn(valueBlock, 1);
            grid.Children.Add(valueBlock);

            btn.Content = grid;
        }

        private void RefreshUI()
        {
            if (_config.Layers.Count <= _currentLayer) return;
            var layer = _config.Layers[_currentLayer];

            // Update buttons
            for (int row = 0; row < _config.Rows; row++)
            {
                for (int col = 0; col < _config.Cols; col++)
                {
                    if (_buttonGrid == null) continue;
                    var cfg = layer.Buttons[row, col];
                    var keyNum = row * _config.Cols + col + 1;
                    UpdateButtonContent(_buttonGrid[row, col], keyNum, cfg.Label, cfg.Mapping);
                }
            }

            // Update knobs
            string[] prefixes = { "↺ CCW", "⏺ Press", "↻ CW" };
            for (int k = 0; k < _config.Knobs && k < _knobUIs.Count; k++)
            {
                var knob = layer.Knobs[k];
                var ui = _knobUIs[k];
                UpdateKnobButtonContent(ui.CCW, prefixes[0], knob.CCW.Label, knob.CCW.Mapping);
                UpdateKnobButtonContent(ui.Press, prefixes[1], knob.Press.Label, knob.Press.Mapping);
                UpdateKnobButtonContent(ui.CW, prefixes[2], knob.CW.Label, knob.CW.Mapping);
            }

            StatusText.Text = $"Layer {_currentLayer + 1} active";
        }

        private void GridButton_Click(object sender, RoutedEventArgs e)
        {
            var btn = (Button)sender;
            var (row, col) = ((int, int))btn.Tag;

            _selectedButton = (row, col);
            _selectedKnob = null;

            var config = _config.Layers[_currentLayer].Buttons[row, col];
            SelectedKeyLabel.Text = $"Button {row * _config.Cols + col + 1}";
            LabelInput.Text = config.Label;
            MappingInput.Text = config.Mapping;

            Log($"Selected Button {row * _config.Cols + col + 1}");
        }

        private void KnobButton_Click(object sender, RoutedEventArgs e)
        {
            var btn = (Button)sender;
            var tagParts = btn.Tag.ToString()!.Split(',');
            var knobIndex = int.Parse(tagParts[0]);
            var action = tagParts[1];

            _selectedButton = null;
            _selectedKnob = (knobIndex, action);

            var knob = _config.Layers[_currentLayer].Knobs[knobIndex];
            var config = action switch
            {
                "ccw" => knob.CCW,
                "press" => knob.Press,
                "cw" => knob.CW,
                _ => throw new Exception("Invalid action")
            };

            SelectedKeyLabel.Text = $"Knob {knobIndex + 1} {action.ToUpper()}";
            LabelInput.Text = config.Label;
            MappingInput.Text = config.Mapping;

            Log($"Selected Knob {knobIndex + 1} {action.ToUpper()}");
        }

        private void QuickPreset_Click(object sender, RoutedEventArgs e)
        {
            var btn = (Button)sender;
            var preset = btn.Tag?.ToString() ?? "";
            MappingInput.Text = preset;
            
            // Also set a default label based on preset
            if (string.IsNullOrEmpty(LabelInput.Text))
            {
                LabelInput.Text = btn.Content?.ToString() ?? "";
            }
        }

        private void ApplyMapping_Click(object sender, RoutedEventArgs e)
        {
            var label = LabelInput.Text.Trim();
            var mapping = MappingInput.Text.Trim().ToLower();

            if (_selectedButton.HasValue)
            {
                var (row, col) = _selectedButton.Value;
                _config.Layers[_currentLayer].Buttons[row, col].Label = label;
                _config.Layers[_currentLayer].Buttons[row, col].Mapping = mapping;
                Log($"✓ Button {row * _config.Cols + col + 1}: {label} → {mapping}");
            }
            else if (_selectedKnob.HasValue)
            {
                var (knobIndex, action) = _selectedKnob.Value;
                var knob = _config.Layers[_currentLayer].Knobs[knobIndex];
                var target = action switch
                {
                    "ccw" => knob.CCW,
                    "press" => knob.Press,
                    "cw" => knob.CW,
                    _ => throw new Exception("Invalid action")
                };
                target.Label = label;
                target.Mapping = mapping;
                Log($"✓ Knob {knobIndex + 1} {action.ToUpper()}: {label} → {mapping}");
            }
            else
            {
                Log("⚠️ Select a button or knob first!");
                return;
            }

            RefreshUI();
            _osdWindow?.RefreshOsd();
        }

        private void LayerSelector_Changed(object sender, SelectionChangedEventArgs e)
        {
            if (_config?.Layers == null || _config.Layers.Count == 0) return;

            _currentLayer = LayerSelector.SelectedIndex;
            _selectedButton = null;
            _selectedKnob = null;
            SelectedKeyLabel.Text = "None";
            LabelInput.Text = "";
            MappingInput.Text = "";
            RefreshUI();
            _osdWindow?.SetLayer(_currentLayer);
            Log($"Switched to Layer {_currentLayer + 1}");
        }

        private async void ReadFromDevice_Click(object sender, RoutedEventArgs e)
        {
            Log("📖 Reading from device...");

            try
            {
                var output = await RunMacrocli("read --all-layers");
                if (string.IsNullOrEmpty(output))
                {
                    Log("❌ No output. Is device connected?");
                    return;
                }

                ParseMacrocliOutput(output);
                BuildDynamicUI();
                RefreshUI();
                
                if (_osdWindow != null)
                {
                    _osdWindow.Config = _config;
                    _osdWindow.RebuildForDevice(_config.Rows, _config.Cols, _config.Knobs);
                    _osdWindow.RefreshOsd(_currentLayer);
                }
                
                Log("✓ Configuration loaded!");
            }
            catch (Exception ex)
            {
                Log($"❌ Error: {ex.Message}");
            }
        }

        private void ParseMacrocliOutput(string output)
        {
            var rowsMatch = Regex.Match(output, @"rows:\s*(\d+)");
            var colsMatch = Regex.Match(output, @"cols:\s*(\d+)");
            var knobsMatch = Regex.Match(output, @"knobs:\s*(\d+)");

            int rows = rowsMatch.Success ? int.Parse(rowsMatch.Groups[1].Value) : 4;
            int cols = colsMatch.Success ? int.Parse(colsMatch.Groups[1].Value) : 4;
            int knobs = knobsMatch.Success ? int.Parse(knobsMatch.Groups[1].Value) : 3;

            Log($"Device: {rows}×{cols} buttons, {knobs} knobs");
            InitializeDefaultConfig(rows, cols, knobs);

            var buttonPattern = @"\(delay:\s*\d+,\s*(?:per_key_delays:\s*\[[^\]]*\],\s*)?mapping:\s*""([^""]*)""\)";
            var matches = Regex.Matches(output, buttonPattern);

            if (matches.Count > 0)
            {
                int matchIndex = 0;
                for (int layer = 0; layer < 3 && matchIndex < matches.Count; layer++)
                {
                    for (int row = 0; row < rows && matchIndex < matches.Count; row++)
                    {
                        for (int col = 0; col < cols && matchIndex < matches.Count; col++)
                        {
                            _config.Layers[layer].Buttons[row, col].Mapping = matches[matchIndex].Groups[1].Value;
                            matchIndex++;
                        }
                    }

                    for (int k = 0; k < knobs && matchIndex + 2 < matches.Count; k++)
                    {
                        _config.Layers[layer].Knobs[k].CCW.Mapping = matches[matchIndex++].Groups[1].Value;
                        _config.Layers[layer].Knobs[k].Press.Mapping = matches[matchIndex++].Groups[1].Value;
                        _config.Layers[layer].Knobs[k].CW.Mapping = matches[matchIndex++].Groups[1].Value;
                    }
                }
            }
        }

        private async void ProgramDevice_Click(object sender, RoutedEventArgs e)
        {
            Log("🚀 Programming device...");

            try
            {
                var ronPath = Path.Combine(Path.GetTempPath(), "macropad_config.ron");
                GenerateRonConfig(ronPath);

                var output = await RunMacrocli($"program -c \"{ronPath}\"");
                Log(output);
                Log("✓ Device programmed!");
            }
            catch (Exception ex)
            {
                Log($"❌ Error: {ex.Message}");
            }
        }

        private void GenerateRonConfig(string path)
        {
            var sb = new StringBuilder();
            sb.AppendLine("(");
            sb.AppendLine("    device: (");
            sb.AppendLine("        orientation: Normal,");
            sb.AppendLine($"        rows: {_config.Rows},");
            sb.AppendLine($"        cols: {_config.Cols},");
            sb.AppendLine($"        knobs: {_config.Knobs},");
            sb.AppendLine("    ),");
            sb.AppendLine("    layers: [");

            foreach (var layer in _config.Layers)
            {
                sb.AppendLine("        (");
                sb.AppendLine("            buttons: [");

                for (int row = 0; row < _config.Rows; row++)
                {
                    sb.Append("                [");
                    for (int col = 0; col < _config.Cols; col++)
                    {
                        var mapping = layer.Buttons[row, col].Mapping ?? "";
                        sb.Append($"(delay: 0, mapping: \"{mapping}\")");
                        if (col < _config.Cols - 1) sb.Append(", ");
                    }
                    sb.AppendLine("],");
                }

                sb.AppendLine("            ],");
                sb.AppendLine("            knobs: [");

                for (int k = 0; k < _config.Knobs; k++)
                {
                    var knob = layer.Knobs[k];
                    sb.AppendLine($"                (ccw: (delay: 0, mapping: \"{knob.CCW.Mapping ?? ""}\"), press: (delay: 0, mapping: \"{knob.Press.Mapping ?? ""}\"), cw: (delay: 0, mapping: \"{knob.CW.Mapping ?? ""}\")),");
                }

                sb.AppendLine("            ],");
                sb.AppendLine("        ),");
            }

            sb.AppendLine("    ],");
            sb.AppendLine(")");

            File.WriteAllText(path, sb.ToString());
        }

        private void SaveConfig_Click(object sender, RoutedEventArgs e)
        {
            var dialog = new SaveFileDialog
            {
                Filter = "JSON Config|*.json|RON Config|*.ron",
                DefaultExt = ".json",
                FileName = "macropad_config"
            };

            if (dialog.ShowDialog() == true)
            {
                if (dialog.FileName.EndsWith(".ron"))
                    GenerateRonConfig(dialog.FileName);
                else
                    SaveAsJson(dialog.FileName);
                Log($"✓ Saved: {dialog.FileName}");
            }
        }

        private void SaveAsJson(string path)
        {
            var json = System.Text.Json.JsonSerializer.Serialize(_config, 
                new System.Text.Json.JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(path, json);
        }

        private void LoadConfig_Click(object sender, RoutedEventArgs e)
        {
            var dialog = new OpenFileDialog { Filter = "Config Files|*.json;*.ron" };

            if (dialog.ShowDialog() == true)
            {
                try
                {
                    if (dialog.FileName.EndsWith(".json"))
                    {
                        var json = File.ReadAllText(dialog.FileName);
                        _config = System.Text.Json.JsonSerializer.Deserialize<MacropadConfig>(json)!;
                    }
                    else
                    {
                        ParseMacrocliOutput(File.ReadAllText(dialog.FileName));
                    }
                    
                    BuildDynamicUI();
                    RefreshUI();
                    Log($"✓ Loaded: {dialog.FileName}");
                }
                catch (Exception ex)
                {
                    Log($"❌ Error: {ex.Message}");
                }
            }
        }

        private async System.Threading.Tasks.Task<string> RunMacrocli(string arguments)
        {
            if (!File.Exists(MacrocliPath))
                throw new FileNotFoundException($"macrocli.exe not found");

            var psi = new ProcessStartInfo
            {
                FileName = MacrocliPath,
                Arguments = arguments,
                UseShellExecute = false,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                CreateNoWindow = true
            };

            using var process = Process.Start(psi)!;
            var output = await process.StandardOutput.ReadToEndAsync();
            var error = await process.StandardError.ReadToEndAsync();
            await process.WaitForExitAsync();

            if (!string.IsNullOrEmpty(error)) Log($"stderr: {error}");
            return output + error;
        }

        private void Log(string message)
        {
            LogOutput.Text = $"[{DateTime.Now:HH:mm:ss}] {message}\n{LogOutput.Text}";
            if (LogOutput.Text.Length > 5000)
                LogOutput.Text = LogOutput.Text.Substring(0, 4000);
        }

        private void ToggleOsd_Click(object sender, RoutedEventArgs e)
        {
            if (_osdWindow == null)
            {
                _osdWindow = new OsdWindow { Config = _config };
                _osdWindow.RebuildForDevice(_config.Rows, _config.Cols, _config.Knobs);
                _osdWindow.Closed += (s, args) => _osdWindow = null;
            }

            if (_osdWindow.IsVisible)
            {
                _osdWindow.Hide();
                Log("👁 OSD hidden");
            }
            else
            {
                _osdWindow.Config = _config;
                _osdWindow.RefreshOsd(_currentLayer);
                _osdWindow.Show();
                Log("👁 OSD shown");
            }
        }

        protected override void OnClosed(EventArgs e)
        {
            _osdWindow?.Close();
            base.OnClosed(e);
        }
    }

    // Data classes
    public class MacropadConfig
    {
        public int Rows { get; set; } = 4;
        public int Cols { get; set; } = 4;
        public int Knobs { get; set; } = 3;
        public List<LayerConfig> Layers { get; set; } = new();
    }

    public class LayerConfig
    {
        public ButtonConfig[,] Buttons { get; set; } = new ButtonConfig[4, 4];
        public KnobConfig[] Knobs { get; set; } = new KnobConfig[3];
    }

    public class ButtonConfig
    {
        public string Mapping { get; set; } = "";
        public string Label { get; set; } = "";
        public int Delay { get; set; } = 0;
    }

    public class KnobConfig
    {
        public ButtonConfig CCW { get; set; } = new();
        public ButtonConfig Press { get; set; } = new();
        public ButtonConfig CW { get; set; } = new();
    }
}
