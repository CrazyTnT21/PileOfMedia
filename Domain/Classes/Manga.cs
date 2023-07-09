using System;
using Domain.Attributes;
using Domain.Interfaces;
using Domain.Schemas;

namespace Domain.Classes;

[DBTable<Manga>(MangaSchema.Table)]
public sealed class Manga : ITranslationFields
{
  [DBColumn(MangaSchema.Id)] public uint PK { get; set; }

  [DBColumn(MangaSchema.Name, TranslationSchema.Table)]
  public string Name { get; set; }

  [DBColumn(MangaSchema.Description, TranslationSchema.Table)]
  public string? Description { get; set; }

  [DBColumn(MangaSchema.Status, TranslationSchema.Table)]
  public string? Status { get; set; }

  [DBColumn(MangaSchema.Chapters)] public ushort? Chapters { get; set; }
  [DBColumn(MangaSchema.Volumes)] public ushort? Volumes { get; set; }
  [DBColumn(MangaSchema.PublishStart)] public DateOnly? PublishStart { get; set; }
  [DBColumn(MangaSchema.PublishEnd)] public DateOnly? PublishEnd { get; set; }
  [DBColumn(MangaSchema.ImageSource)] public string? ImageSource { get; set; }
  [DBColumn(MangaSchema.AverageScore)] public float? AverageScore { get; set; }

  public TranslationField[]? TranslationFields { get; set; }
}
