program Sum;
var
  lb, sum, ub;
begin
  read(lb, ub);
  sum := 0;
  while lb <= ub do begin
    sum := sum + lb;
    lb := lb + 1
  end;
  write(sum)
end