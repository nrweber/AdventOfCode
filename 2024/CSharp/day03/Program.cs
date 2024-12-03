using System.Text.RegularExpressions;

void Part1(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);
   

    var sum = 0;
    foreach(var line in lines)
    {
        var matches = Regex.Matches(line, @"mul\(\d{1,3},\d{1,3}\)");

        foreach(Match match in matches)
        {
            var func = match.Value;
            func = func.Substring(4, func.Length-5);

            var split = func.Split(",");

            int one = Int32.Parse(split[0]);
            int two = Int32.Parse(split[1]);

            sum += one * two;
        }
    }

    Console.WriteLine(sum);

}

void Part2(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);

    var sum = 0;
    bool enabled = true;
    foreach(var line in lines)
    {
        var matches = Regex.Matches(line, @"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)");
        

        foreach(Match match in matches)
        {
            var func = match.Value;
            if(func == "don't()")
            {
                enabled = false;
            }
            else if(func == "do()")
            {
                enabled = true;
            }
            else
            {
                if(enabled)
                {
                    func = func.Substring(4, func.Length-5);

                    var split = func.Split(",");

                    int one = Int32.Parse(split[0]);
                    int two = Int32.Parse(split[1]);

                    sum += one * two;
                }
            }
        }
    }

    Console.WriteLine(sum);
}


Console.WriteLine("Part1 Example:");
Part1("example.txt");
Console.WriteLine("");

Console.WriteLine("Part1:");
Part1("input.txt");
Console.WriteLine("");

Console.WriteLine("Part2 Example:");
Part2("example2.txt");
Console.WriteLine("");

Console.WriteLine("Part2:");
Part2("input.txt");
