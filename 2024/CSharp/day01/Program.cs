void Part1(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);

    List<int> one = new();
    List<int> two = new();

    foreach(var line in lines)
    {
        var split = line.Split(" ", StringSplitOptions.RemoveEmptyEntries);
        one.Add(Int32.Parse(split[0]));
        two.Add(Int32.Parse(split[1]));
    }

    one.Sort();
    two.Sort();

    int total = 0;
    for(int i = 0; i < one.Count; i++)
    {
        total += Math.Abs(one[i] - two[i]);
    }
    Console.WriteLine(total);
}

void Part2(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);

    List<int> one = new();
    Dictionary<int, int> counts = new();

    foreach(var line in lines)
    {
        var split = line.Split(" ", StringSplitOptions.RemoveEmptyEntries);
        one.Add(Int32.Parse(split[0]));

        int sideTwo = Int32.Parse(split[1]);
        if(!counts.ContainsKey(sideTwo))
            counts[sideTwo] = 0;
        counts[sideTwo] += 1;
    }
    

    int total = 0;
    foreach(var i in one)
    {
        if(counts.ContainsKey(i))
            total += i*counts[i];
    }
    Console.WriteLine(total);

}


Console.WriteLine("Part1 Example:");
Part1("example.txt");
Console.WriteLine("Should be: 11");
Console.WriteLine("");

Console.WriteLine("Part1:");
Part1("input.txt");
Console.WriteLine("");

Console.WriteLine("Part2 Example:");
Part2("example.txt");
Console.WriteLine("Should be: 31");
Console.WriteLine("");

Console.WriteLine("Part2:");
Part2("input.txt");
