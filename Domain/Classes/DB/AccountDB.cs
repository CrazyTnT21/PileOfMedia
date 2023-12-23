using System.ComponentModel.DataAnnotations;
using Domain.Interfaces;

namespace Domain.Classes.DB;

public sealed class AccountDB: IEntity
{
  public int Id { get; set; }
  [StringLength(255)] public string Email { get; set; } = null!;
  [StringLength(48)] public string Password { get; set; } = null!;
  public int FKUser { get; set; }
}
