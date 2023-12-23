using System.ComponentModel.DataAnnotations;
using Domain.Enums;

namespace Domain.Classes.DB;

public sealed class ThemeTranslationDB : ITranslationDB
{
  [StringLength(50)] public string Name { get; set; } = null!;
  public int FKTranslation { get; set; }
  public Language Language { get; set; }
}
