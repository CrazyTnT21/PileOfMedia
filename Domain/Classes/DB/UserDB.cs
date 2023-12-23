using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class UserDB: IEntity
{
  public int Id { get; set; }
  public string Name { get; set; } = null!;
  public string? Description { get; set; }
  public int? FKProfilePicture { get; set; }
  public bool Deleted { get; set; }
  public DateOnly Joined { get; set; }
}
