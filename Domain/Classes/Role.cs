using Domain.Interfaces;

namespace Domain.Classes;

public sealed class Role: IEntity
{
  public int Id { get; set; }
  public string Name { get; set; } = null!;
}
