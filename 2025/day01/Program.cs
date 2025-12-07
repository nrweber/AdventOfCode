void Part1(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);
    
    int zeroCount = 0;
    int dialPosition = 50;

    foreach(var line in lines)
    {
        int num = Int32.Parse(line[1..]);

        if(line[0] == 'R')
            dialPosition += num;
        else
            dialPosition -= num;
        dialPosition = dialPosition%100;

        if(dialPosition == 0)
            zeroCount++;

    }

    Console.WriteLine($"Password: {zeroCount}");
}

void Part2(string filename)
{
    var lines = System.IO.File.ReadAllLines(filename);

    int zeroCount = 0;
    int dialPosition = 50;

    foreach(var line in lines)
    {
        int num = Int32.Parse(line[1..]);

        if(line[0] == 'R')
        {
            /*
            //brute force
            for(int i = 0; i < num; i++)
            {
                dialPosition += 1;
                if(dialPosition == 100)
                    dialPosition = 0;
                if(dialPosition == 0)
                    zeroCount++;
            }
            */


            dialPosition += num;
            while(dialPosition > 99)
            {
                dialPosition -= 100;
                zeroCount++;
            }
        }
        else
        {
            /*
            //brute force
            for(int i = 0; i < num; i++)
            {
                dialPosition -= 1;
                if(dialPosition == -1)
                    dialPosition = 99;
                if(dialPosition == 0)
                    zeroCount++;
            }
            */

            // This is to prevent double counting of zero
            // if the dial is already on 0
            if(dialPosition == 0)
                dialPosition += 100;

            dialPosition -= num;
            while(dialPosition < 0)
            {
                dialPosition += 100;
                zeroCount++;
            }
            if(dialPosition == 0)
                zeroCount++;
        }

        //Console.WriteLine($"{line} -- num: {num} -- dial: {dialPosition} -- zero_count: {zeroCount}");


    }

    Console.WriteLine($"Password: {zeroCount}");

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
