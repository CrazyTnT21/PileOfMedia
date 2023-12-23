using Domain.Interfaces;

namespace Domain.Classes;

public sealed class User: IEntity
{
  public int Id { get; set; }
  public string Name { get; set; } = null!;
  public DateOnly Joined { get; set; }
  public string? Description { get; set; }
  public Image? ProfilePicture { get; set; }
  public bool Deleted { get; set; }
}

public sealed class CreateUser
{
  public int Id { get; set; }
  public string? Name { get; set; } = null!;
  public string? Description { get; set; }
  public CreateImage? ProfilePicture { get; set; }
}
