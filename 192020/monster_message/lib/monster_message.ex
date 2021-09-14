defmodule MonsterMessage do
  @moduledoc """
  Documentation for `MonsterMessage`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> MonsterMessage.hello()
      :world

  """
  def recurse_number(node_map, value) do
    IO.puts inspect " value " <> inspect value
    cond do
      Regex.match?(~r/\|/, value) ->
        [left_tree, right_tree] = String.split(value, "|", trim: true)
        # seems premature to split it here and process first..
        # all the | are OR while space " " are AND
        # need to call recurse_number for another recursion

        left_message = recurse_number(node_map, left_tree)
        right_message = recurse_number(node_map, right_tree)

        # IO.puts "L msg " <> inspect left_message

        # IO.puts "R msg " <> inspect right_message

        [left_message, right_message]
      Regex.match?(~r/\s/, value) ->
        nodes = String.split(String.trim(value), " ", trim: true)
        # IO.puts inspect "Nodes " <> inspect nodes
        Enum.reduce(nodes, [] , fn node, acc ->
          temp = [recurse_number(node_map, node)]
          # IO.puts "ACC & #{inspect acc} #{inspect temp}"
          # here needs to & (join / permutate)
          p = unless Enum.empty?(acc) do
            permute_list(acc, temp)
          else
            acc ++ temp
          end
          # IO.puts "ACC ret #{inspect p}"
          p
        end)
      value=="a" || value=="b" ->
        value
      true ->
        recurse_number(node_map, node_map[value])
      end
  end

  # && function
  # given 2 lists, return the whole permutation range
  def permute_list(list1, list2) do
    l1 = List.flatten(list1)
    l2 = List.flatten(list2)
    Enum.reduce(l1, [], fn item, acc ->
      item_list = for r <- l2 do
        item <> r
      end
      acc ++ item_list
    end)
  end

  defp split_on_colon(item, acc) do
    [node_index, next_node_combi] = String.split(item, ":", trim: true)

    Map.put(acc, node_index, String.trim(next_node_combi))
  end

  def main do
    node_map = File.stream!("input_1") |> Enum.reduce(%{}, &split_on_colon/2)

    final_list = recurse_number(node_map, "0")

    msg_list = File.stream!("input_msg") |> Enum.reduce([], fn item, acc ->
     acc ++ [String.trim(item)]
    end)

    # IO.puts inspect final_list
    # IO.puts inspect msg_list

    MapSet.intersection(MapSet.new(final_list), MapSet.new(msg_list))
    |> MapSet.size

  end
end
