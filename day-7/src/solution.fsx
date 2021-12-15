let input = (System.IO.File.ReadAllText "input.txt" ).Split(',') |> Seq.map System.Int32.Parse

let part1 input = 
    {Seq.min input .. Seq.max input}
    |> Seq.map (fun x -> Seq.fold (fun sum position -> sum + abs (position - x)) 0 input) 
    |> Seq.min
printfn "Part one solution %d" (part1 input)

let part2 input = 
    {Seq.min input .. Seq.max input}
    |> Seq.map (fun x -> Seq.fold (fun sum position -> sum + ({0..abs (position - x)} |> Seq.sum)) 0 input) 
    |> Seq.min
printfn "Part two solution %d" (part2 input)
