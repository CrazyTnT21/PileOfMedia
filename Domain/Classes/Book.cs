using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Classes;

public sealed class Book : IEntity, IUserStats, IAdded
{
  public int Id { get; set; }
  public string Title { get; set; } = null!;
  public string? Description { get; set; }
  public short? Chapters { get; set; }
  public short? Pages { get; set; }
  public int? Words { get; set; }
  public DateOnly? Published { get; set; }
  public Image Cover { get; set; } = null!;
  public decimal Score { get; set; }
  public DateOnly Added { get; set; }
  public int Popularity { get; set; }
  public int Favorites { get; set; }
  public int Members { get; set; }
  public int Rank { get; set; }

  public Genre[] Genres { get; set; } = Array.Empty<Genre>();
  public Theme[] Themes { get; set; } = Array.Empty<Theme>();
}

public sealed class CreateBook
{
  public int Id { get; set; }
  public short? Chapters { get; set; }
  public short? Pages { get; set; }
  public int? Words { get; set; }
  public DateOnly? Published { get; set; }
  public CreateImage? Cover { get; set; } = null!;

  public Genre[]? Genres { get; set; } = Array.Empty<Genre>();
  public Theme[]? Themes { get; set; } = Array.Empty<Theme>();

  public Dictionary<Language, TitleDescription>? Translations { get; set; } = null!;
}
