defmodule Grid do
    def remove_available_rolls(grid) do
        new_grid = grid
        |> Enum.map(fn {point, g} ->
            if g != "@" do
            {
                point,
                g
            }
            else
                occupied_neighbours = Grid.neighbors(point)
                    |> Enum.map(fn p -> case Map.get(grid, p, ".") do
                        "@" -> 1
                        _ -> 0
                        end
                    end)
                    |> Enum.sum();
                new_grapheme = if occupied_neighbours < 4, do: "x", else: g;
                {
                    point,
                    new_grapheme
                }
            end
        end)
        |> Map.new()
        if new_grid == grid do
            grid
         else
            remove_available_rolls(new_grid)
         end
    end
    def parse_input(input) do
        input
        |> String.split("\n")
        |> Enum.with_index
        |> Enum.flat_map(fn {x, row} ->
            x |> String.graphemes
            |> Enum.with_index
            |> Enum.map(fn {grapheme, col} -> {{row, col}, grapheme} end)
        end)
        |> Map.new
    end
    def print(grid) do # inverse of parse function
        grid
        |> Enum.sort_by(fn {{r, c}, _} -> r*100 + c end)
        |> Enum.group_by(fn {{r, _}, _} -> r end)
        |> Enum.map(fn {_, row} ->
            row
            |> Enum.map(fn {{_, _}, g} -> g end)
            |> List.to_string()
        end)
        |> Enum.join("\n")
    end
    def neighbors(point) do
        grid = for i <- [-1, 0, 1], j <- [-1, 0, 1], {i,j} != {0,0}, do: {i, j};
        grid |> Enum.map(fn delta -> Grid.vec_add(delta, point) end)
    end
    def vec_add({a, b}, {c, d}), do: {a+c, b+d}
end
done = File.read!("input")
    |> Grid.parse_input()
    |> Grid.remove_available_rolls()
done
    |> Grid.print()
    |> IO.puts()
done
    |> Enum.count(fn {_, g} -> g == "x"end)
    |> IO.puts()
