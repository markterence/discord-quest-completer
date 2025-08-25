using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace dummy_window
{
    public partial class Form1 : Form
    {
        private string _theme = "light";
        private string _title_args = "name";

        public string theme
        {
            get => _theme;
            set
            {
                _theme = value.ToLower(); 
                ApplyTheme();
            }
        }

        public string TitleArg
        {
            get => _title_args;
            set
            {
                _title_args = value;
            }
        }

        public Form1()
        {
            InitializeComponent();
        }

        private void ApplyTheme()
        {
            bool dark = _theme == "dark";
            BackColor = dark ? Color.FromArgb(30, 30, 30) : SystemColors.Control;
            ForeColor = dark ? Color.Gainsboro : SystemColors.ControlText;

            foreach (Control c in Controls)
            {
                c.BackColor = BackColor;
                c.ForeColor = ForeColor;
                if (c is LinkLabel link)
                {
                    if (dark)
                    {
                        link.LinkColor = Color.DeepSkyBlue;
                        link.ActiveLinkColor = Color.LightBlue;
                        link.VisitedLinkColor = Color.MediumPurple;
                        link.DisabledLinkColor = Color.Gray;
                    }
                    else
                    {
                        link.LinkColor = Color.Blue;
                        link.ActiveLinkColor = Color.Red;
                        link.VisitedLinkColor = Color.Purple;
                        link.DisabledLinkColor = SystemColors.GrayText;
                    }
                }
            }
        }

        private void Form1_Load(object sender, EventArgs e)
        {
            label1.Text = TitleArg + "\r\n" + "Discord Quest Completer Window";
        }

        private void linkLabel1_LinkClicked(object sender, LinkLabelLinkClickedEventArgs e)
        {
            try
            {
                VisitLink();
            }
            catch (Exception ex)
            {
                MessageBox.Show("Unable to open link that was clicked.");
            }
        }

        public void VisitLink()
        {
            linkLabel1.LinkVisited = true;
            System.Diagnostics.Process.Start("https://github.com/markterence/discord-quest-completer");
        }
    }
}
