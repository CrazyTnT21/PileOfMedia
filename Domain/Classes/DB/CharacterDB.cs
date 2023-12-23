using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class CharacterDB: IEntity
{
  public int Id { get; set; }
  public DateOnly? Birthday { get; set; }
  public int? Height { get; set; }
  public int? FKImage { get; set; }
}
