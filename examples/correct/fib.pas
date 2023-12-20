
program fibonacci;

const index := 30;

var return,i,a;
procedure fib(a,x);

var sum;
begin
  sum := 0;
  if x<2 then
    return := x
  else
    begin
      call fib(a+1,x-1);
      sum := sum+return;
      call fib(a+1,x-2);
      sum := sum+return;
      return := sum
    end
end

begin
  i := 1;
  a := 2;
  while i<=index do
    begin
      call fib(a+1,i);
      write(return);
      i := i+1
    end
end
