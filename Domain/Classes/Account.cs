using Domain.Attributes;
using Domain.Schemas;

namespace Domain.Classes;

[DBTable<Account>(AccountSchema.Table)]
public sealed class Account
{
  [DBColumn(AccountSchema.FKUser, UserSchema.Table)]
  // public User User { get; set; }
  public uint FKUser { get; set; }

  [DBColumn(AccountSchema.Email)] public string Email { get; set; } = null!;
  [DBColumn(AccountSchema.Password)] public string Password { get; set; } = null!;
}
