using System.ComponentModel.DataAnnotations;
using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class CompanyDB: IEntity
{
  public int Id { get; set; }
  [StringLength(100)] public string Name { get; set; } = null!;
  public int? FKLogo { get; set; }
}
