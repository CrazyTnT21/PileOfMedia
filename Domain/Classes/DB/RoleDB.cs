using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class RoleDB: IEntity
{
  public int Id { get; set; }
}
