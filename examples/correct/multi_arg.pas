program Add;
const index := 20;
var return,a,b,c,sum;

procedure add(a,b,c);
var sum;
begin
  write(index);
  return := a+b+c
end;

procedure addClosure(a,b,c);
const index := 1;
begin
  write(index);
  sum := 3
end

begin
  read(a,b,c);
  call add(b+a,a,c);
  call addClosure(a,b,c);
  write(return);
  write(sum)
end