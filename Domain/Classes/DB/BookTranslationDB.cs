using System.ComponentModel.DataAnnotations;
using Domain.Enums;

namespace Domain.Classes.DB;

public sealed class BookTranslationDB: ITranslationDB
{
  [StringLength(150)] public string Title { get; set; } = null!;
  [StringLength(500)] public string? Description { get; set; }
  public int FKTranslation { get; set; }
  public Language Language { get; set; }
}
