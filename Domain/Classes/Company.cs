using Domain.Interfaces;

namespace Domain.Classes;

public sealed class Company: IEntity
{
  public int Id { get; set; }
  public string Name { get; set; } = null!;
  public Image Logo { get; set; } = null!;
}
