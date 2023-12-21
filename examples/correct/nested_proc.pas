program NestedProc;
  var cnt;

  procedure a();
    var cnt;

    procedure aa();
    begin
      cnt := cnt + 1;
      write(cnt)
    end
  
  begin
    cnt := cnt + 1;
    write(cnt);
    call aa()
  end;

  procedure b();
  begin
    cnt := cnt + 1;
    write(cnt)
  end

begin
  cnt := 3;
  call a();
  call b()
end