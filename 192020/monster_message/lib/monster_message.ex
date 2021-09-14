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
  def split_on_colon(item, acc) do
    [node_index, next_node_combi] = String.split(item, ":", trim: true)

    Map.put(acc, node_index, String.trim(next_node_combi))
  end

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

        # left_nodes = String.split(String.trim(left_tree), " ", trim: true)
        # right_nodes = String.split(String.trim(right_tree), " ", trim: true)

        # left_message = Enum.reduce(left_nodes, [], fn node, acc ->
        #   acc ++ [recurse_number(node_map, node)]
        # end)

        # left_message = if Enum.all?(left_message, fn x -> x == "a" || x == "b" end) do
        #   Enum.join(left_message)
        # else
        #   left_message
        # end
        IO.puts "L msg " <> inspect left_message

        # right_message = Enum.reduce(right_nodes, [], fn node, acc ->
        #   acc ++ [recurse_number(node_map, node)]
        # end)

        # right_message = if Enum.all?(right_message, fn x -> x == "a" || x == "b" end) do
        #   Enum.join(right_message)
        # else
        #   right_message
        # end
        IO.puts "R msg " <> inspect right_message

        [left_message, right_message]
      Regex.match?(~r/\s/, value) ->
        nodes = String.split(String.trim(value), " ", trim: true)
        IO.puts inspect "Nodes " <> inspect nodes
        Enum.reduce(nodes, [] , fn node, acc ->
          temp = [recurse_number(node_map, node)]
          IO.puts "ACC & #{inspect acc} #{inspect temp}"
          # here needs to & (join / permutate)
          p = unless Enum.empty?(acc) do
            permute_list(acc, temp)
          else
            acc ++ temp
          end
          IO.puts "ACC ret #{inspect p}"
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

  def permute_list2(list1, list2) do
    Enum.reduce(list1, [], fn item, acc ->
      item_list = for r <- list2 do
        item <> r
      end
      acc ++ item_list
    end)
  end

  def main do
    node_map = File.stream!("input_t") |> Enum.reduce(%{}, &split_on_colon/2)

   recurse_number(node_map, "0")

  end
end
