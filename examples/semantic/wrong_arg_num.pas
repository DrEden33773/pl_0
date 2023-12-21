program MultiDef;

var a;

procedure proc();
begin
  write(1)
end;

procedure procc(x, t, z);
begin
  write(1)
end

begin
  call proc(1, 1, 1);
  call procc(3)
end