defmodule JurassicJigsaw do
  @moduledoc """
  Documentation for `JurassicJigsaw`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> JurassicJigsaw.hello()
      :world

  """
  defmodule Tile do
    defstruct side_score: 0, index: 0, sides: []
  end

  def flip_vertical(%Tile{sides: sides} = t) do
    # swap top and bottom
    # reverse of left, reverse of right
    [top, bottom, left, right] = sides

    Map.replace(t, :sides, [bottom, top, String.reverse(left), String.reverse(right)])
  end

  def flip_horizontal(%Tile{sides: sides} = t) do
    # swap left and right
    # reverse of top, reverse of bottom
    [top, bottom, left, right] = sides

    Map.replace(t, :sides, [String.reverse(top), String.reverse(bottom), right, left])
  end

  def rotate_right(%Tile{sides: sides} = t) do
    # top -> right, bottom -> left
    # reverse(right) -> bottom, reverse(left) -> top
    [top, bottom, left, right] = sides

    Map.replace(t, :sides, [String.reverse(left), String.reverse(right), bottom, top])
  end

  def convert_to_tiles(jigsaw_map) do
    for mp <- jigsaw_map do
      [{mp_ind, mp_val}] = Map.to_list(mp)
      top = List.first(mp_val)
      bottom = List.last(mp_val)
      left = List.foldl(mp_val, "", fn x, acc ->
        acc <> String.first(x)
      end)
      right = List.foldl(mp_val, "", fn x, acc ->
        acc <> String.last(x)
      end)
      %Tile{index: mp_ind, sides: [top, bottom, left, right]}
    end
  end

  def put_into_map(item, acc) do
    cond do
      key = Regex.run(~r/^Tile (\d+):/, item, capture: :all_but_first) ->
        [index] = key
        index = String.to_integer(index)

        [ %{index => []} | acc ]
      (item =~ ~r/^\n$/) -> acc
      true ->
        [mp | tail] = acc
        [mp_ind] = Map.keys(mp)
        new_mp_val = mp[mp_ind] ++ [String.trim(item)]
        [ %{mp_ind => new_mp_val} | tail ]
    end
    # if tile, create index in acc map
    # if space, do nothing
    # else (anything in between), add into value of the acc map
  end
  # 1. get the 4 sides of each tile piece => [T, B, L, R]
  # 2. for each tile piece
  # 3. what are the combinations of changes? =>
  #     -> [nothing + Flip V, nothing + Flip H, Rotate + Flip V, Rotate + Flip H ...]
  #     -> First rotate.. how many time?
  #     -> first do the flip vertical
  # 4. how to compare?
  #     -> Top to Bottom, Right to Left, Left to Right, Bottom to Top
  # 5. if any matches, increase side_score
  # 6. go back to step 3, try different combination
  # 7. finish after looping through the entire list (other than itself), go to next tile piece
  def main do
    jigsaw_map = File.stream!("input_t") |> Enum.reduce([], &put_into_map/2)

    tile_map = convert_to_tiles(jigsaw_map)

  end
end
