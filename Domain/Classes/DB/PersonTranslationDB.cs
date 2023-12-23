using System.ComponentModel.DataAnnotations;
using Domain.Enums;

namespace Domain.Classes.DB;

public sealed class PersonTranslationDB : ITranslationDB
{
  [StringLength(500)] public string Description { get; set; } = null!;
  public int FKTranslation { get; set; }
  public Language Language { get; set; }
}
