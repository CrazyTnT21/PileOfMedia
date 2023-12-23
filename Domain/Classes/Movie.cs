using Domain.Classes.DB;
using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Classes;

public sealed class Movie : IEntity, IUserStats, IAdded
{
  public int Id { get; set; }
  public string Title { get; set; } = null!;
  public string? Description { get; set; }
  public DateOnly? Airing { get; set; }
  public TimeSpan? Length { get; set; }
  public decimal Score { get; set; }
  public Image Cover { get; set; } = null!;
  public DateOnly Added { get; set; }
  public int Popularity { get; set; }
  public int Favorites { get; set; }
  public int Members { get; set; }
  public int Rank { get; set; }

  public Genre[] Genres { get; set; } = Array.Empty<Genre>();
  public Theme[] Themes { get; set; } = Array.Empty<Theme>();
}

public sealed class CreateMovie
{
  public int Id { get; set; }
  public DateOnly? Airing { get; set; }
  public TimeSpan? Length { get; set; }
  public CreateImage? Cover { get; set; }

  public Genre[]? Genres { get; set; } = Array.Empty<Genre>();
  public Theme[]? Themes { get; set; } = Array.Empty<Theme>();

  public Dictionary<Language, TitleDescription>? Translations { get; set; } = null!;

  public MovieDB MapToDB()
  {
    return new MovieDB()
    {
      Id = Id,
      Airing = Airing,
      Length = Length,
      FKCover = Cover!.Id
    };
  }
}
