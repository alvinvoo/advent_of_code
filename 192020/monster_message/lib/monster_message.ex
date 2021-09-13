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

  def recurse_number(node_map, node_index) do
    value = node_map[node_index]
    IO.puts inspect "node index " <> node_index <> " value " <> value
    if value=="a" || value=="b" do
      [value]
    else
      if Regex.match?(~r/\|/, value) do
        [left_tree, right_tree] = String.split(value, "|", trim: true)
        left_nodes = String.split(String.trim(left_tree), " ", trim: true)
        right_nodes = String.split(String.trim(right_tree), " ", trim: true)

        left_message = Enum.reduce(left_nodes, [], fn node, acc ->
          acc = acc ++ recurse_number(node_map, node)
          IO.puts "acc L msg " <> inspect acc
          acc
        end)

        left_message = if Enum.all?(left_message, fn x -> x == "a" || x == "b" end) do
          Enum.join(left_message)
        else
          left_message
        end

        right_message = Enum.reduce(right_nodes, [], fn node, acc ->
          acc = acc ++ recurse_number(node_map, node)
          IO.puts "acc R msg " <> inspect acc
          acc
        end)

        right_message = if Enum.all?(right_message, fn x -> x == "a" || x == "b" end) do
          Enum.join(right_message)
        else
          right_message
        end

        [[left_message, right_message]]
      else
        nodes = String.split(String.trim(value), " ", trim: true)
        IO.puts inspect "Nodes " <> inspect nodes
        Enum.reduce(nodes, [] , fn node, acc ->
          acc ++ recurse_number(node_map, node)
        end)
        # need to be able to join here
      end
    end
  end


  # given 2 lists, return the whole permutation range
  def permute_list(list1, list2) do
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
