[ranges, ingredients] = File.read!("input1")
|> String.split("\n\n")
|> Enum.map(&String.split(&1, "\n"))
|> Enum.with_index()
|> Enum.map(fn
  {ranges, 0} -> ranges |> Enum.map(fn r ->
    r |> String.split("-")
    |> Enum.map(&String.to_integer(&1))
    |> List.to_tuple()
  end)
  {ingredient, 1} -> ingredient |> Enum.map(&String.to_integer(&1))
end)

ranges = ranges
|> Enum.sort()
|> Enum.reduce([], fn
  x, [] -> [x]
  {c, d}, [{a, b} | t] ->
    #because the list is sorted, c > a
    if b >= c-1 do [{min(a, c), max(b,d)}|t] # c is between a and b, so they must be overlapping
    else [ {c,d}| [{a, b}|t]] end # they are sepperate
end)
|> IO.inspect()


p1 = ingredients
|> Enum.count(fn x -> Enum.any?(ranges, fn {a, b} -> a<=x && x<=b end) end)
p2 = ranges
|> Enum.map(fn {a, b} -> b-a+1 end)
|> Enum.sum()

IO.puts("p1: #{p1}, p2: #{p2}")
