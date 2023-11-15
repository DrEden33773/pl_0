program OnePlusTwo;
var
  a, b, c, bool;
begin
  a := 1;
  b := 2;
  c := a + b;
  write(c);
  write(c + 1);
  if c <> 3 then write(0) else write(1)
end