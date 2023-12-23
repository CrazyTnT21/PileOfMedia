using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Classes;

public sealed class Game : IEntity, IUserStats, IAdded
{
  public int Id { get; set; }
  public string Title { get; set; } = null!;
  public string? Description { get; set; }
  public DateOnly? Published { get; set; }
  public decimal Score { get; set; }
  public Image Cover { get; set; } = null!;
  public DateOnly Added { get; set; }
  public int Popularity { get; set; }
  public int Favorites { get; set; }
  public int Members { get; set; }
  public int Rank { get; set; }
}

public sealed class CreateGame
{
  public int Id { get; set; }
  public DateOnly? Published { get; set; }
  public CreateImage? Cover { get; set; }

  public Dictionary<Language, TitleDescription>? Translations { get; set; } = null!;
}
