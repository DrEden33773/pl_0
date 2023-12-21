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
  if odd (c + 1) then write(1) else write(0);
  if c <> 5 then write(1) else write(0);
  if c > 7 then write(1) else write(0);
  if c >= 0 then write(1) else write(0);
  if c <= 10 then write(1) else write(0);
  if c <> 0 then write(111)
end