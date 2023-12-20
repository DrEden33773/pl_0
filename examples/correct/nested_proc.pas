program NestedProc;

var identifier;

procedure a();
  procedure aa();
  begin
    identifier := identifier + 1;
    write(identifier)
  end
begin
  identifier := identifier + 1;
  write(identifier);
  call aa()
end;

procedure b();
begin
  identifier := identifier + 1;
  write(identifier)
end

begin
  identifier := 3;
  call a();
  call b()
end