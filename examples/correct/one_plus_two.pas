program OnePlusTwo;
var
  a, b, c, bool;
begin
  a := 1;
  b := 2;
  c := a + b;
  write(c);
  write(c + 1);
  if c = 3 then write(1) else write(0);
  if c = 4 then write(1) else write(0);
  if odd (c + 1) then write(1) else write(0)
end