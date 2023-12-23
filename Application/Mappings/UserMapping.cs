using Application.DBMapping;
using Domain.Classes;
using Domain.ValueObjects;

namespace Application.Mappings;

public sealed class UserMapping : IMapping
{
  public IDBMapping Mapping { get; } =
    new DBMapping<User>(true)
      .Column(nameof(User.Id), "Id")
      .Column(nameof(User.Name), "Name")
      .Column(nameof(User.Description), "Description")
      .Column(nameof(User.Joined), "Joined")
      .Column(nameof(User.Deleted), "Deleted")
      .Join(nameof(User.ProfilePicture),
            new Join("Image", "User",
                     new Condition(
                       new TableColumn("Image", "Id"),
                       new TableColumn("User", "FKProfilePicture"))));
}
