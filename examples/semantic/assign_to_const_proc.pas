program AssignToConstProc;
const i := 1;

procedure proc();
begin
  write(i)
end

begin
  i := 16;
  proc := 16
end