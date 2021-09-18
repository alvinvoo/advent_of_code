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
    #IO.puts inspect " value " <> inspect value
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
          #IO.puts "ACC ret #{inspect p}"
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

  def split_on_colon(item, acc) do
    [node_index, next_node_combi] = String.split(item, ":", trim: true)

    Map.put(acc, node_index, String.trim(next_node_combi))
  end

  def main do
    node_map = File.stream!("input_2_t_recurse") |> Enum.reduce(%{}, &split_on_colon/2)

    final_list = recurse_number(node_map, "0")

    msg_list = File.stream!("input_2_msg") |> Enum.reduce([], fn item, acc ->
     acc ++ [String.trim(item)]
    end)

    IO.puts "Output\n"
     IO.puts inspect final_list
     IO.puts inspect Enum.count(final_list)

    #  min = Enum.min(final_list, fn a, b -> String.length(a) <= String.length(b) end)
    #  max = Enum.max(final_list, fn a, b -> String.length(a) >= String.length(b) end)
    #  IO.puts min
    #  IO.puts String.length(min)
    #  IO.puts max
    #  IO.puts String.length(max)
    # IO.puts inspect msg_list

    MapSet.intersection(MapSet.new(final_list), MapSet.new(msg_list))
    |> MapSet.size

  end

  # below is for part 2
  def part2 do
    sa = small_fun_export_array()

    msg_list = File.stream!("input_msg") |> Enum.reduce([], fn item, acc ->
     acc ++ [String.trim(item)]
    end)

    r = for msg <- msg_list do
      spt_str = split_string(msg)
      #IO.puts "checking #{msg} #{inspect spt_str}"

      result = Enum.reduce(spt_str, false, fn chunk_list, acc ->
        acc || msg_check(sa, chunk_list)
      end)
      #IO.puts "result for #{msg} is #{result} \n\n"
      result
    end

    Enum.count(r, &(&1==true))
  end

  def small_fun_export_array do
    node_map = File.stream!("input_1") |> Enum.reduce(%{}, &split_on_colon/2)

    part_42 = List.flatten(recurse_number(node_map, "42"))
    part_31 = List.flatten(recurse_number(node_map, "31"))
    {part_42, part_31}
  end

  def msg_check({part_42, part_31}, {heads, tail}) do

    # for example
    # part_31 = [
    #   "bbaba", "bbbaa", "babab", "babaa", "babba", "baaba", "baaab",
    #   "ababb", "abaab", "abbab", "abaaa", "abbaa", "abbba", "aabab", "aabaa", "aabba"
    # ]

    # part_42 = [
    #   "babbb", "baabb", "bbaab", "bbabb", "bbbab", "bbbbb", "abbbb", "aabbb", "aaaab", "aaabb",
    #   "aaaba", "ababa", "bbbba", "aaaaa", "baaaa", "bbaaa"
    # ]

    #h_m = MapSet.intersection(MapSet.new(part_42), MapSet.new(heads))
    #IO.puts "heads match #{inspect h_m}"

    contain_heads = MapSet.subset?(MapSet.new(heads), MapSet.new(part_42))
    #IO.puts "contain all heads? #{inspect heads} #{contain_heads}"

    #t_m = MapSet.intersection(MapSet.new(part_31), MapSet.new(tail))
    #IO.puts "tail match #{inspect t_m}"

    contain_tail = MapSet.subset?(MapSet.new(tail), MapSet.new(part_31))
    #IO.puts "contain all tails? #{inspect tail} #{contain_tail}"

    contain_heads && contain_tail
  end

  def split_string(str) do
    whole_list = String.graphemes(str)
    |> Enum.chunk_every(8)
    |> Enum.map(&Enum.join/1)

    list_count = Enum.count(whole_list)
    p1 = {Enum.slice(whole_list, 0..(list_count-2)), Enum.slice(whole_list, (list_count-1)..list_count)}

    cond do
      list_count >= 11 ->
        p2 = {Enum.slice(whole_list, 0..(list_count-3)), Enum.slice(whole_list, (list_count-2)..list_count)}
        p3 = {Enum.slice(whole_list, 0..(list_count-4)), Enum.slice(whole_list, (list_count-3)..list_count)}
        p4 = {Enum.slice(whole_list, 0..(list_count-5)), Enum.slice(whole_list, (list_count-4)..list_count)}
        p5 = {Enum.slice(whole_list, 0..(list_count-6)), Enum.slice(whole_list, (list_count-5)..list_count)}
        [p1, p2, p3, p4, p5]
      list_count >= 9 ->
        p2 = {Enum.slice(whole_list, 0..(list_count-3)), Enum.slice(whole_list, (list_count-2)..list_count)}
        p3 = {Enum.slice(whole_list, 0..(list_count-4)), Enum.slice(whole_list, (list_count-3)..list_count)}
        p4 = {Enum.slice(whole_list, 0..(list_count-5)), Enum.slice(whole_list, (list_count-4)..list_count)}
        [p1, p2, p3, p4]
      list_count >= 7 ->
        p2 = {Enum.slice(whole_list, 0..(list_count-3)), Enum.slice(whole_list, (list_count-2)..list_count)}
        p3 = {Enum.slice(whole_list, 0..(list_count-4)), Enum.slice(whole_list, (list_count-3)..list_count)}
        [p1, p2, p3]
      list_count >= 5 ->
        p2 = {Enum.slice(whole_list, 0..(list_count-3)), Enum.slice(whole_list, (list_count-2)..list_count)}
        [p1, p2]
      true -> [p1]
    end

  end
end
