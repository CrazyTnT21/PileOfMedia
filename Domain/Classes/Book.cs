using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using Domain.Attributes;
using Domain.Enums;
using Domain.Interfaces;

namespace Domain.Classes;

[Table("Book")]
public class Book : IEntity
{
  [Key]
  [DatabaseGenerated(DatabaseGeneratedOption.Identity)]
  [Column("Id")]
  public uint Id { get; set; }

  [DBJoin(typeof(Translations), "FKName")]
  [DBAssociation(Key.Language)]
  public string Name { get; set; } = null!;

  [DBJoin(typeof(Translations), "FKDescription")]
  [DBAssociation(Key.Language)]
  public string? Description { get; set; }

  [Column("Chapters")] public byte? Chapters { get; set; }
  [Column("Pages")] public ushort? Pages { get; set; }
  [Column("Words")] public uint? Words { get; set; }
  [Column("PublishDate")] public DateOnly PublishDate { get; set; }

  [Column("ImageSource")]
  [StringLength(255)]
  public string? ImageSource { get; set; }

  [Column("AverageScore")] public decimal AverageScore { get; set; }
  [Column("Added")] public DateOnly Added { get; set; }
}
