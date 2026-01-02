#p1
File.read!("input")
|> String.split("\n")
|> Enum.map(&String.split(&1))
|> Enum.reverse()
|> Enum.zip()
|> Enum.map(fn x ->
  [op | tl] = x |> Tuple.to_list()
  tl = tl |> Enum.map(&String.to_integer(&1))
  case op do
    "+" -> tl |> Enum.sum()
    "*" -> tl |> Enum.product()
  end
  end)
|> Enum.sum()
|> IO.inspect()

#p2
File.read!("input")
|> String.split("\n")
|> Enum.map(&String.graphemes(&1))
|> Enum.zip()
|> Enum.reverse()
|> Enum.flat_map(fn x ->
  x = x
  |> Tuple.to_list()
  |> List.to_string()
  cond do
    String.ends_with?(x, "+") -> [String.slice(x, 0..-2//1), "+"]
    String.ends_with?(x, "*") -> [String.slice(x, 0..-2//1), "*"]
    true -> [x]
  end
end)
|> Enum.map(&String.trim(&1))
|> Enum.chunk_by(fn x -> x !="" end)
|> Enum.filter(fn x -> x != [""] end)
|> Enum.map(&Enum.reverse(&1))
|> Enum.map(fn x ->
  [op | tl] = x
  tl = tl |> Enum.map(&String.to_integer(&1))
  case op do
    "+" -> tl |> Enum.sum()
    "*" -> tl |> Enum.product()
  end
  end)
|> Enum.sum()
|> IO.inspect()
