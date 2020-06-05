using System;
using Newtonsoft.Json;

namespace Reproto
{
    class Program
    {
        static void Main(string[] args)
        {
            string line;
            while ((line = Console.ReadLine()) != null) {
                Test.Entry foo = JsonConvert.DeserializeObject<Test.Entry>(line);
                Console.Out.Write("#<>" + JsonConvert.SerializeObject(foo) + "\n");
                Console.Out.Flush();
            }
        }
    }
}
