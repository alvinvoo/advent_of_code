defmodule UniversalOrbitMap do
  def split_and_reduce(item, acc) do
   [orbitee, orbiter] = String.split(item, ")", trim: true)

    Map.put(acc, String.trim_trailing(orbiter), orbitee)
  end

  def count_orbit(orbit_map, orbiter, count \\ 0) do
    if orbit_map[orbiter] == "COM" do
      count + 1
    else
      count_orbit(orbit_map, orbit_map[orbiter], count + 1)
    end
  end

  def count_orbit_2(orbit_map, source, target, count \\ 0) do
    if orbit_map[source] == target do
      count + 1
    else
      count_orbit_2(orbit_map, orbit_map[source], target, count + 1)
    end
  end

  def store_orbit_path(orbit_map, source, orbit_path \\ []) do
    if orbit_map[source] == "COM" do
      orbit_path
    else
      next_source = orbit_map[source]
      store_orbit_path(orbit_map,
      next_source,
      orbit_path ++ [next_source])
    end
  end

  def main() do
    orbit_map = File.stream!("input.txt")
              |> Enum.reduce(%{}, &split_and_reduce/2)
    # part 1
    # Enum.sum(for {orbiter, _} <- orbit_map do
    #   count_orbit(orbit_map, orbiter)
    # end)

    # part 2:
    # 1. store the paths of YOU and SAN to COM respectively
    # 2. find the intersection object
    # 3. count the orbits for YOU and SAN to that object
    # 4. add together and minus 2

    you_path = store_orbit_path(orbit_map, "YOU")
    san_path = store_orbit_path(orbit_map, "SAN")
    both_paths = you_path ++ san_path

    freq = Enum.frequencies(both_paths)

    intersection_obj = Enum.reduce_while(san_path, "", fn x, acc ->
     if freq[x] != 2, do: {:cont , acc}, else: {:halt, x}
    end)

    you_count = count_orbit_2(orbit_map, "YOU", intersection_obj)
    san_count = count_orbit_2(orbit_map, "SAN", intersection_obj)

    you_count + san_count - 2

  end
end
