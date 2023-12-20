program Test;
  const a := 5;
  var j, sum, x; // 1, _, 10

  procedure Clojure(x); // 6
    var j; // 1
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
  read(x, j); // 10, 1
  call Clojure(j + 5);
  write(j)
end