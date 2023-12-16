program Test;
  const a := 5;
  var j, sum, x;

  procedure Clojure(x);
    var j;
  begin
    j := 1;
    sum := 0;
    while j <= x do begin
      sum := sum + j;
      j := j + 1
    end;
    write(sum)
  end

begin
  read(x, j);
  call Clojure(j + 5);
  write(j)
end