using System.ComponentModel.DataAnnotations;
using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class PersonDB: IEntity
{
  public int Id { get; set; }
  [StringLength(100)] public string Name { get; set; } = null!;
  [StringLength(50)] public string? FirstName { get; set; }
  [StringLength(50)] public string? LastName { get; set; }
  public DateOnly? Birthday { get; set; }
  public int? Height { get; set; }
  public int? FKImage { get; set; }
}
