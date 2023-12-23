using Application.DBMapping;
using Domain.Classes;
using Domain.ValueObjects;

namespace Application.Mappings;

public sealed class AccountMapping : IMapping
{
  public IDBMapping Mapping { get; } =
    new DBMapping<Account>()
      .Column(nameof(Account.Password), "Password")
      .Column(nameof(Account.Email), "EMail")
      .Join(new Join("User", true, "Account",
                     new Condition(new TableColumn("User", "Id"),
                                   new TableColumn("Account", "FKUser"))));
}
