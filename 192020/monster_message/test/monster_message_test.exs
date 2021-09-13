defmodule MonsterMessageTest do
  use ExUnit.Case
  doctest MonsterMessage

  test "greets the world" do
    assert MonsterMessage.hello() == :world
  end
end
