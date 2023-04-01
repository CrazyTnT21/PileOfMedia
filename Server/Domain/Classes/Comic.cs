using Domain.Attributes;
using Domain.Interfaces;
using Domain.Schemas;

namespace Domain.Classes;

[DBTable<Comic>(ComicSchema.Table)]
public sealed class Comic : ITranslationFields
{
  [DBColumn(ComicSchema.Id)] public uint PK { get; set; }

  [DBColumn("Name", TranslationSchema.Table)]
  public string Name { get; set; } = null!;

  [DBColumn("Description", TranslationSchema.Table)]
  public string? Description { get; set; }

  [DBColumn("Status", TranslationSchema.Table)]
  public string? Status { get; set; }

  [DBColumn(ComicSchema.Chapters)] public ushort? Chapters { get; set; }
  [DBColumn(ComicSchema.Volumes)] public ushort? Volumes { get; set; }
  [DBColumn(ComicSchema.PublishStart)] public DateOnly? PublishStart { get; set; }
  [DBColumn(ComicSchema.PublishEnd)] public DateOnly? PublishEnd { get; set; }
  [DBColumn(ComicSchema.ImageSource)] public string? ImageSource { get; set; }
  [DBColumn(ComicSchema.AverageScore)] public decimal? AverageScore { get; set; }

  [DBForeign("ComicXCharacter", "PK", "FKComic")]
  public Character[]? characters { get; set; }

  [DBForeign("ComicXCreator", "PK", "FKComic")]
  public Creator[]? creators { get; set; }

  public TranslationField[]? TranslationFields { get; set; }
}
