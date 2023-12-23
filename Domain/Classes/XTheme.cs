namespace Domain.Classes;

public sealed class XTheme<T>
{
  public T Item { get; set; } = default!;
  public Theme Role { get; set; } = null!;
}
