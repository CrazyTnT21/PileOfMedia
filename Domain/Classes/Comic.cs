using System;
using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using Domain.Attributes;
using Domain.Interfaces;
using Domain.Schemas;

namespace Domain.Classes;

[Table(ComicSchema.Table)]
public sealed class Comic: IEntity
{
  [Key]
  [DatabaseGenerated(DatabaseGeneratedOption.Identity)]
  [Column(ComicSchema.Id)]
  public uint Id { get; set; }

  [DBJoin(typeof(Translations), "FKName")]
  [DBAssociation(Key.Language)]
  public string Name { get; set; } = null!;

  [DBJoin(typeof(Translations), "FKDescription")]
  [DBAssociation(Key.Language)]
  public string Description { get; set; } = null!;

  [DBJoin(typeof(Status), "FKStatus")]
  [ForeignKey("FKStatus")]
  public Status Status { get; set; } = null!;

  [Column(ComicSchema.Chapters)] public ushort? Chapters { get; set; }
  [Column(ComicSchema.Volumes)] public ushort? Volumes { get; set; }
  [Column(ComicSchema.PublishStart)] public DateOnly? PublishStart { get; set; }
  [Column(ComicSchema.PublishEnd)] public DateOnly? PublishEnd { get; set; }

  [Column(ComicSchema.ImageSource)]
  [StringLength(255)]
  public string? ImageSource { get; set; }

  [Column(ComicSchema.AverageScore)] public decimal? AverageScore { get; set; }

  // [NotMapped]
  // [DBMany<Genre>("ComicXGenre", "FKComic")]
  // public Genre[]? Genres { get; set; }
  //
  // [NotMapped]
  // [DBMany<Character>("ComicXCharacter", "FKComic")]
  // public Character[]? Characters { get; set; }
  //
  // [NotMapped]
  // [DBMany<Creator>("ComicXCreator", "FKComic")]
  // public Creator[]? Creators { get; set; }

  [NotMapped] public TranslationField[]? LanguageFields { get; set; }
}
