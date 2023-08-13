using System.ComponentModel.DataAnnotations.Schema;

namespace Domain.Classes.DB;

public class BookDB
{
  [Column("Id")] public uint Id { get; set; }
  [Column("FKName")] public string Name { get; set; } = null!;
  [Column("FKDescription")] public string? Description { get; set; }
  [Column("Chapters")] public byte? Chapters { get; set; }
  [Column("Pages")] public ushort? Pages { get; set; }
  [Column("Words")] public uint? Words { get; set; }
  [Column("PublishDate")] public DateOnly PublishDate { get; set; }
  [Column("ImageSource")] public string? ImageSource { get; set; }
  [Column("AverageScore")] public decimal AverageScore { get; set; }
  [Column("Added")] public DateOnly Added { get; set; }
}
