using System;
using Domain.Attributes;
using Domain.Schemas;

namespace Domain.Classes;

[DBTable<Game>(GameSchema.Table)]
public sealed class Game
{
  [DBColumn(GameSchema.Id)] public uint PK { get; set; }
  [DBColumn(GameSchema.Name)] public string Name { get; set; } = null!;
  [DBColumn(GameSchema.Description)] public string? Description { get; set; }
  [DBColumn(GameSchema.Published)] public DateOnly? Published { get; set; }
  [DBColumn(GameSchema.AverageScore)] public float? AverageScore { get; set; }
  [DBColumn(GameSchema.ImageSource)] public string? ImageSource { get; set; }

  [DBColumn(GameSchema.Status, TranslationSchema.Table)]
  public string? Status { get; set; }
}
