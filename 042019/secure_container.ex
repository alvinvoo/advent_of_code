
defmodule SecureContainer do
  def reduce_same(digits) do
    Enum.flat_map_reduce(digits, 0, fn x, acc ->
      if acc != nil && x >= acc
      do {[x == acc], x}
      else {:halt, nil}
      end
    end)
  end

  def check_any_double(filter_arr) do
    Enum.any?(filter_arr)
  end

  # for part 2
  def check_special_double(filter_arr) do
    # only 5 combinations from this 6 members list
    # any one of them is correct means correct
    # 0..2: [false, true, false]
    # 1..3: [false, true, false]
    # 2..4: [false, true, false]
    # 3..5: [false, true, false]
    # 4..5: [false, true]

    Enum.slice(filter_arr, 0..2) == [false, true, false]
    || Enum.slice(filter_arr, 1..3) == [false, true, false]
    || Enum.slice(filter_arr, 2..4) == [false, true, false]
    || Enum.slice(filter_arr, 3..5) == [false, true, false]
    || Enum.slice(filter_arr, 4..5) == [false, true]
  end

  def check(number) do
    Integer.digits(number)
    |> reduce_same
    |> then(fn {filter_arr, result} ->
     if result == nil
     do false
     else check_special_double filter_arr
     end
    end)
  end

  def main do
    final_count = 171309..643603
    |> Enum.filter(fn x -> check x end) |> Enum.count

    IO.puts(final_count)
  end
end
