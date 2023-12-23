using Domain.Interfaces;

namespace Domain.Classes;

public sealed class Theme: IEntity
{
  public int Id { get; set; }
  public string Name { get; set; } = null!;
}
