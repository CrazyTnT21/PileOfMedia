using System.ComponentModel.DataAnnotations.Schema;
using Domain.Schemas;

namespace Domain.Classes.DB;

public class ComicDB
{
  [Column(ComicSchema.Id)] public uint Id { get; set; }
  [Column(ComicSchema.FKName)] public string Name { get; set; } = null!;
  [Column(ComicSchema.FKDescription)] public string FKDescription { get; set; } = null!;
  [Column(ComicSchema.FKStatus)] public uint FKStatus { get; set; }
  [Column(ComicSchema.Chapters)] public ushort? Chapters { get; set; }
  [Column(ComicSchema.Volumes)] public ushort? Volumes { get; set; }
  [Column(ComicSchema.PublishStart)] public DateOnly? PublishStart { get; set; }
  [Column(ComicSchema.PublishEnd)] public DateOnly? PublishEnd { get; set; }
  [Column(ComicSchema.ImageSource)] public string? ImageSource { get; set; }
  [Column(ComicSchema.AverageScore)] public decimal? AverageScore { get; set; }
}
