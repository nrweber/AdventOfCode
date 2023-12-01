

void Part1(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);
    var total = 0;
    foreach(var line in lines)
    {
        for(int i = 0; i < line.Length; i++)
        {
            var end = i+1;
            if(Int32.TryParse(line[i..end], out int result))
            {
                total += result * 10;
                break;
            }
        }
        for(int i = line.Length-1; i >= 0; i--)
        {
            var end = i+1;
            if(Int32.TryParse(line[i..end], out int result))
            {
                total += result;
                break;
            }
        }
    }

    Console.WriteLine(total);
}

void Part2(string filename)
{
    Dictionary<string, int> mapping = new() { 
        { "one", 1 }, 
        { "two", 2 }, 
        { "three", 3 }, 
        { "four", 4 }, 
        { "five", 5 }, 
        { "six", 6 }, 
        { "seven", 7 }, 
        { "eight", 8 }, 
        { "nine", 9 }, 
        { "0", 0 }, 
        { "1", 1 }, 
        { "2", 2 }, 
        { "3", 3 }, 
        { "4", 4 }, 
        { "5", 5 }, 
        { "6", 6 }, 
        { "7", 7 }, 
        { "8", 8 }, 
        { "9", 9 }, 
    };
    var lines = System.IO.File.ReadAllLines(filename);

    var total = 0;
    foreach(var line in lines)
    {
        for(int i = 0; i < line.Length; i++)
        {
            foreach( var item in mapping)
            {
                if(line[i..line.Length].StartsWith(item.Key))
                {
                    total += item.Value * 10;
                    i = line.Length;
                    break;
                }
            }
        }
        for(int i = line.Length-1; i >= 0; i--)
        {
            foreach( var item in mapping)
            {
                if(line[i..line.Length].StartsWith(item.Key))
                {
                    total += item.Value;
                    i = -1;
                    break;
                }
            }
        }
    }

    Console.WriteLine(total);
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
