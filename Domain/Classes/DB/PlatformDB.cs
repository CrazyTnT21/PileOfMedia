using System.ComponentModel.DataAnnotations;
using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class PlatformDB: IEntity
{
  public int Id { get; set; }
  [StringLength(50)] public string Name { get; set; } = null!;
  [StringLength(10)] public string? ShortName { get; set; }
  public int FKCompany { get; set; }
  public int? FKLogo { get; set; }
}
