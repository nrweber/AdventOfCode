void Part1(string filename)
{
    long ans = 0;
    
    var lines = System.IO.File.ReadAllLines(filename);

    var ranges = lines[0].Split(",", StringSplitOptions.RemoveEmptyEntries);
    foreach(var range in ranges)
    {
        var s = range.Split("-");
        long lower = Int64.Parse(s[0]);
        long upper = Int64.Parse(s[1]);

        for(long i = lower; i <= upper; i++)
        {
            var i_str = i.ToString();
            if(i_str.Length % 2 == 0)
            {
                var mid = i_str.Length/2;
                if(i_str[0..mid] == i_str[mid..])
                    ans += i;
            }
        }
    }
    Console.WriteLine(ans);

}

void Part2(string filename)
{
    long ans = 0;

    var lines = System.IO.File.ReadAllLines(filename);

    var ranges = lines[0].Split(",", StringSplitOptions.RemoveEmptyEntries);
    foreach(var range in ranges)
    {
        var s = range.Split("-");
        long lower = Int64.Parse(s[0]);
        long upper = Int64.Parse(s[1]);

        for(long i = lower; i <= upper; i++)
        {
            if(is_part2_valid(i))
                ans += i;
        }
    }
    Console.WriteLine(ans);
}

bool is_part2_valid(long num)
{
    var i_str = num.ToString();
    for(int d = 2; d <= i_str.Length; d++)
    {
        if(i_str.Length % d == 0)
        {
            bool match = true;
            var part_length = i_str.Length/d;
            for(var x = 0; x < i_str.Length; x+=part_length)
            {
                if( i_str.Substring(x, part_length) != i_str.Substring(0, part_length))
                    match = false;
            }

            if(match)
                return true;
        }
    }
    return false;
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
