using System.ComponentModel.DataAnnotations;
using Domain.Enums;

namespace Domain.Classes.DB;

public sealed class CharacterTranslation : ITranslationDB
{
  [StringLength(150)] public string Name { get; set; } = null!;
  [StringLength(50)] public string? FirstName { get; set; }
  [StringLength(50)] public string? LastName { get; set; }
  [StringLength(500)] public string? Description { get; set; }

  public int FKTranslation { get; set; }
  public Language Language { get; set; }
}
