using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Controls.Primitives;
using System.Windows.Input;
using System.Windows.Media;

namespace MacropadGUI
{
    public partial class OsdWindow : Window
    {
        private Dictionary<string, string> _labels = new();
        private string _labelsPath;
        private Button[,]? _osdButtons;
        private List<(Button ccw, Button press, Button cw)> _knobButtons = new();
        private int _currentLayer = 0;
        private double _currentOpacity = 0.9;
        
        private int _rows = 4;
        private int _cols = 4;
        private int _knobs = 3;
        
        public MacropadConfig? Config { get; set; }
        public event Action<int, int, int, string>? LabelChanged;

        public OsdWindow()
        {
            InitializeComponent();
            _labelsPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "labels.json");
            LoadLabels();
        }

        private void LoadLabels()
        {
            try
            {
                if (File.Exists(_labelsPath))
                {
                    var json = File.ReadAllText(_labelsPath);
                    _labels = JsonSerializer.Deserialize<Dictionary<string, string>>(json) ?? new();
                }
            }
            catch { _labels = new(); }
        }

        private void SaveLabels()
        {
            try
            {
                var json = JsonSerializer.Serialize(_labels, new JsonSerializerOptions { WriteIndented = true });
                File.WriteAllText(_labelsPath, json);
            }
            catch { }
        }

        public void RebuildForDevice(int rows, int cols, int knobs)
        {
            _rows = rows;
            _cols = cols;
            _knobs = knobs;
            
            // Resize window based on device
            int baseWidth = Math.Max(400, cols * 90 + (knobs > 0 ? 160 : 0));
            int baseHeight = Math.Max(280, rows * 70 + 80);
            this.Width = baseWidth;
            this.Height = baseHeight;

            BuildDynamicGrid();
            BuildDynamicKnobs();
        }

        private void BuildDynamicGrid()
        {
            OsdButtonGrid.Rows = _rows;
            OsdButtonGrid.Columns = _cols;
            OsdButtonGrid.Children.Clear();
            _osdButtons = new Button[_rows, _cols];

            for (int row = 0; row < _rows; row++)
            {
                for (int col = 0; col < _cols; col++)
                {
                    var keyNum = row * _cols + col + 1;
                    var btn = CreateOsdButton(("button", row, col), $"Button {keyNum}");
                    _osdButtons[row, col] = btn;
                    OsdButtonGrid.Children.Add(btn);
                }
            }
        }

        private void BuildDynamicKnobs()
        {
            // Clear existing knobs
            Knob1Grid.Children.Clear();
            Knob2Grid.Children.Clear();
            Knob3Grid.Children.Clear();
            _knobButtons.Clear();

            // Hide all knob panels first
            var knobPanels = new[] { Knob1Panel, Knob2Panel, Knob3Panel };
            foreach (var panel in knobPanels)
                panel.Visibility = Visibility.Collapsed;

            if (_knobs == 0)
            {
                KnobsColumn.Width = new GridLength(0);
                return;
            }

            KnobsColumn.Width = new GridLength(150);
            var knobGrids = new[] { Knob1Grid, Knob2Grid, Knob3Grid };
            
            for (int k = 0; k < _knobs && k < 3; k++)
            {
                knobPanels[k].Visibility = Visibility.Visible;
                
                string[] actionNames = { "ccw", "press", "cw" };
                Button ccw = null!, press = null!, cw = null!;

                for (int a = 0; a < 3; a++)
                {
                    var btn = CreateOsdButton(("knob", k, actionNames[a]), $"Knob {k + 1} {actionNames[a].ToUpper()}");
                    btn.MinHeight = 45;
                    
                    if (a == 0) ccw = btn;
                    else if (a == 1) press = btn;
                    else cw = btn;
                    
                    knobGrids[k].Children.Add(btn);
                }
                
                _knobButtons.Add((ccw, press, cw));
            }
        }

        private Button CreateOsdButton(object tag, string tooltip = "")
        {
            var btn = new Button
            {
                Tag = tag,
                Margin = new Thickness(2),
                Background = new SolidColorBrush(Color.FromArgb(0xAA, 0x0f, 0x34, 0x60)),
                Foreground = new SolidColorBrush(Color.FromRgb(0xea, 0xea, 0xea)),
                BorderBrush = new SolidColorBrush(Color.FromRgb(0xe9, 0x45, 0x60)),
                BorderThickness = new Thickness(1),
                FontSize = 9,
                Cursor = Cursors.Hand,
                Padding = new Thickness(2),
                HorizontalContentAlignment = HorizontalAlignment.Center,
                VerticalContentAlignment = VerticalAlignment.Center,
                ToolTip = tooltip
            };
            btn.MouseRightButtonUp += OsdButton_RightClick;
            btn.Click += OsdButton_Click;
            return btn;
        }

        public void RefreshOsd(int layer = -1)
        {
            if (layer >= 0) _currentLayer = layer;
            LayerIndicator.Text = $" [Layer {_currentLayer + 1}] ({_rows}x{_cols})";
            
            if (Config?.Layers == null || Config.Layers.Count <= _currentLayer)
                return;

            var layerConfig = Config.Layers[_currentLayer];

            // Refresh buttons
            for (int row = 0; row < _rows && row < layerConfig.Buttons.GetLength(0); row++)
            {
                for (int col = 0; col < _cols && col < layerConfig.Buttons.GetLength(1); col++)
                {
                    if (_osdButtons == null) continue;
                    var mapping = layerConfig.Buttons[row, col]?.Mapping ?? "";
                    var key = $"{_currentLayer}_btn_{row}_{col}";
                    var label = _labels.TryGetValue(key, out var lbl) ? lbl : "";
                    var keyNum = row * _cols + col + 1;
                    
                    _osdButtons[row, col].Content = CreateButtonContent(keyNum.ToString(), label, mapping);
                }
            }

            // Refresh knobs
            string[] actionSymbols = { "↺", "⏺", "↻" };
            string[] actionNames = { "ccw", "press", "cw" };
            
            for (int k = 0; k < _knobs && k < _knobButtons.Count && k < layerConfig.Knobs.Length; k++)
            {
                var knobConfig = layerConfig.Knobs[k];
                ButtonConfig[] configs = { knobConfig.CCW, knobConfig.Press, knobConfig.CW };
                Button[] buttons = { _knobButtons[k].ccw, _knobButtons[k].press, _knobButtons[k].cw };
                
                for (int action = 0; action < 3; action++)
                {
                    var mapping = configs[action]?.Mapping ?? "";
                    var key = $"{_currentLayer}_knob_{k}_{actionNames[action]}";
                    var label = _labels.TryGetValue(key, out var l) ? l : "";
                    
                    buttons[action].Content = CreateButtonContent(actionSymbols[action], label, mapping);
                }
            }
        }

        private StackPanel CreateButtonContent(string prefix, string label, string mapping)
        {
            var content = new StackPanel { VerticalAlignment = VerticalAlignment.Center };
            
            content.Children.Add(new TextBlock
            {
                Text = prefix,
                FontWeight = FontWeights.Bold,
                FontSize = 10,
                Foreground = new SolidColorBrush(Color.FromRgb(0xe9, 0x45, 0x60)),
                HorizontalAlignment = HorizontalAlignment.Center
            });

            if (!string.IsNullOrEmpty(label))
            {
                content.Children.Add(new TextBlock
                {
                    Text = label,
                    FontSize = 9,
                    FontWeight = FontWeights.SemiBold,
                    Foreground = new SolidColorBrush(Color.FromRgb(0xea, 0xea, 0xea)),
                    TextWrapping = TextWrapping.Wrap,
                    TextAlignment = TextAlignment.Center
                });
            }

            var displayMapping = mapping.Length > 15 ? mapping.Substring(0, 12) + "..." : mapping;
            if (string.IsNullOrEmpty(displayMapping)) displayMapping = "---";
            
            content.Children.Add(new TextBlock
            {
                Text = displayMapping,
                FontSize = 8,
                Foreground = new SolidColorBrush(Color.FromRgb(0x66, 0x66, 0x66)),
                TextWrapping = TextWrapping.Wrap,
                TextAlignment = TextAlignment.Center
            });

            return content;
        }

        private void OsdButton_Click(object sender, RoutedEventArgs e)
        {
            var btn = (Button)sender;
            var originalBg = btn.Background;
            btn.Background = new SolidColorBrush(Color.FromRgb(0xe9, 0x45, 0x60));
            
            var timer = new System.Windows.Threading.DispatcherTimer { Interval = TimeSpan.FromMilliseconds(150) };
            timer.Tick += (s, args) => { btn.Background = originalBg; timer.Stop(); };
            timer.Start();
        }

        private void OsdButton_RightClick(object sender, MouseButtonEventArgs e)
        {
            var btn = (Button)sender;
            var tag = btn.Tag;
            
            string key, title, mapping = "";
            
            if (tag is ValueTuple<string, int, int> buttonTag && buttonTag.Item1 == "button")
            {
                var row = buttonTag.Item2;
                var col = buttonTag.Item3;
                key = $"{_currentLayer}_btn_{row}_{col}";
                title = $"Button {row * _cols + col + 1}";
                mapping = Config?.Layers[_currentLayer]?.Buttons[row, col]?.Mapping ?? "";
            }
            else if (tag is ValueTuple<string, int, string> knobTag && knobTag.Item1 == "knob")
            {
                var knob = knobTag.Item2;
                var action = knobTag.Item3;
                key = $"{_currentLayer}_knob_{knob}_{action}";
                title = $"Knob {knob + 1} {action.ToUpper()}";
                var knobConfig = Config?.Layers[_currentLayer]?.Knobs[knob];
                mapping = action switch
                {
                    "ccw" => knobConfig?.CCW?.Mapping ?? "",
                    "press" => knobConfig?.Press?.Mapping ?? "",
                    "cw" => knobConfig?.CW?.Mapping ?? "",
                    _ => ""
                };
            }
            else return;

            var currentLabel = _labels.TryGetValue(key, out var lbl) ? lbl : "";
            
            var dialog = new EditLabelDialog(0, currentLabel, mapping)
            {
                Owner = this,
                WindowStartupLocation = WindowStartupLocation.CenterOwner
            };
            dialog.Title = title;
            
            if (dialog.ShowDialog() == true)
            {
                _labels[key] = dialog.NewLabel;
                SaveLabels();
                RefreshOsd();
            }
        }

        private void Border_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
        {
            if (e.ButtonState == MouseButtonState.Pressed)
                DragMove();
        }

        private void ToggleOpacity_Click(object sender, RoutedEventArgs e)
        {
            _currentOpacity = _currentOpacity > 0.5 ? 0.3 : 0.9;
            this.Opacity = _currentOpacity;
        }

        private void Close_Click(object sender, RoutedEventArgs e)
        {
            this.Hide();
        }

        public void SetLayer(int layer)
        {
            _currentLayer = layer;
            RefreshOsd();
        }
    }
}
