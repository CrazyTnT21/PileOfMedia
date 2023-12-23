namespace Domain.Classes;

public sealed class XGenre<T>
{
  public T Item { get; set; } = default!;
  public Genre Role { get; set; } = null!;
}
