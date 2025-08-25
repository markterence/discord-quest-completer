using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace dummy_window
{
    internal static class Program
    {
        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        static void Main(string[] args)
        {

            string title = "Dummy Window (runner)";
            string theme = "dark";

            for (int i = 0; i < args.Length; i++)
            {
                if (args[i] == "--title" && i + 1 < args.Length)
                {
                    title = args[i + 1];
                    i++;
                }

                if (args[i] == "--theme" && i + 1 < args.Length) 
                { 
                    theme = args[i + 1];
                    i++;
                }
            }

            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);


            Form1 form = new Form1();
            form.Text = title;
            form.Width = 640;
            form.Height = 480;
            form.theme = theme; 

            Application.Run(form);
        }
    }
}
