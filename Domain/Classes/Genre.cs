using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Classes;

public sealed class Genre : IEntity
{
  public int Id { get; set; }
  public string Name { get; set; } = null!;
}

public sealed class CreateGenre
{
  public int Id { get; set; }
  public Dictionary<Language, TitleDescription>? Translations { get; set; } = null!;
}
