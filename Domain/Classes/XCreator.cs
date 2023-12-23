namespace Domain.Classes;

public sealed class XCreator<T>
{
  public T Item { get; set; } = default!;
  public Role Role { get; set; } = null!;
  public Person Person { get; set; } = null!;
}
