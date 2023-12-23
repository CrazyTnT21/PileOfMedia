using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Classes;

public sealed class GraphicNovel : IEntity, IUserStats, IAdded
{
  public int Id { get; set; }
  public string Title { get; set; } = null!;
  public string Description { get; set; } = null!;
  public Status Status { get; set; }
  public short? Chapters { get; set; }
  public short? Volumes { get; set; }
  public DateOnly? PublishStart { get; set; }
  public DateOnly? PublishEnd { get; set; }
  public Image Cover { get; set; } = null!;
  public decimal Score { get; set; }
  public DateOnly Added { get; set; }

  public Genre[] Genres { get; set; } = Array.Empty<Genre>();
  public Character[] Characters { get; set; } = Array.Empty<Character>();
  public Creator[] Creators { get; set; } = Array.Empty<Creator>();

  public int Popularity { get; set; }
  public int Favorites { get; set; }
  public int Members { get; set; }
  public int Rank { get; set; }
}

public sealed class CreateGraphicNovel
{
  public int Id { get; set; }
  public Status? Status { get; set; }
  public short? Chapters { get; set; }
  public short? Volumes { get; set; }
  public DateOnly? PublishStart { get; set; }
  public DateOnly? PublishEnd { get; set; }
  public CreateImage? Cover { get; set; } = null!;

  public Genre[]? Genres { get; set; } = Array.Empty<Genre>();
  public Character[]? Characters { get; set; } = Array.Empty<Character>();
  public Creator[]? Creators { get; set; } = Array.Empty<Creator>();

  public Dictionary<Language, TitleDescription>? Translations { get; set; } = null!;
}
