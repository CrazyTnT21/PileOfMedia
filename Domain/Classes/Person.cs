using Domain.Interfaces;

namespace Domain.Classes;

public class Person: IEntity
{
  public int Id { get; set; }
  public string Name { get; set; } = null!;
  public string? FirstName { get; set; }
  public string? LastName { get; set; }
  public string? Description { get; set; }
  public DateOnly? Birthday { get; set; }
  public short? Height { get; set; }
  public Image? Image { get; set; }
}
