require './native.so'

begin
  Console.new.freak_out
rescue => e
  p e
end

begin
  Console.new.freak_out
rescue => e
  p e
end
