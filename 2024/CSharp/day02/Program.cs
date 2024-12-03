void Part1(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);
    

    var count = lines.Length;
    foreach(var line in lines)
    {
        List<int> split = line.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(x => Int32.Parse(x)).ToList();
        
        if(split[0] < split[1])
        {
            for(int i = 0; i < split.Count-1; i++)
            {
                if(split[i] >= split[i+1] || split[i+1] - split[i] > 3)
                {
                    count -= 1;
                    break;
                }
            }
        }
        else
        {
            for(int i = 0; i < split.Count-1; i++)
            {
                if(split[i] <= split[i+1] || split[i] - split[i+1] > 3)
                {
                    count -= 1;
                    break;
                }
            }
        }
    }

    Console.WriteLine(count);

}

void Part2(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);
    

    var count = lines.Length;
    foreach(var line in lines)
    {
        int numBads = 0;
        List<int> full = line.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(x => Int32.Parse(x)).ToList();

        for(int remove = 0; remove < full.Count; remove++)
        {
            List<int> split = line.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(x => Int32.Parse(x)).ToList();
            split.RemoveAt(remove);

            if(split[0] < split[1])
            {
                for(int i = 0; i < split.Count-1; i++)
                {
                    if(split[i] >= split[i+1] || split[i+1] - split[i] > 3)
                    {
                        numBads += 1;
                        break;
                    }
                }
            }
            else
            {
                for(int i = 0; i < split.Count-1; i++)
                {
                    if(split[i] <= split[i+1] || split[i] - split[i+1] > 3)
                    {
                        numBads += 1;
                        break;
                    }
                }
            }
        }

        if(numBads == full.Count)
        {
            count -= 1;
        }
    }

    Console.WriteLine(count);
}


Console.WriteLine("Part1 Example:");
Part1("example.txt");
Console.WriteLine("");

Console.WriteLine("Part1:");
Part1("input.txt");
Console.WriteLine("");

Console.WriteLine("Part2 Example:");
Part2("example.txt");
Console.WriteLine("");

Console.WriteLine("Part2:");
Part2("input.txt");
