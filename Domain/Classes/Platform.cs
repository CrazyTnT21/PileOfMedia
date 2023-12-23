using Domain.Interfaces;

namespace Domain.Classes;

public sealed class Platform: IEntity
{
  public int Id { get; set; }
  public string Name { get; set; } = null!;
  public string? ShortName { get; set; }
  public Company Company { get; set; } = null!;
  public Image Logo { get; set; } = null!;
}
