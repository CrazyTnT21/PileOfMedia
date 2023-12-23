using Application.DBMapping;
using Domain.Classes;
using Domain.Classes.DB;
using Npgsql;

namespace Application.Repositories;

public sealed class UserRepository : Repository, IUserRepository
{
  public UserRepository(NpgsqlConnection connection) : base(connection)
  {
  }

  public async Task<User?> GetById(uint id)
  {
    var result = await new Select<User>()
      .Where("Deleted", false)
      .Where("Id", id)
      .UniqueResult(_connection);

    return result;
  }

  public async Task<User?> GetByIdDeleted(uint id)
  {
    var result = await new Select<User>()
      .Where("Id", id)
      .UniqueResult(_connection);

    return result;
  }

  public async Task<IEnumerable<User>> Get()
  {
    List<User> results = await new Select<User>()
      .Where("Deleted", false)
      .List(_connection);

    return results;
  }

  public async Task<IEnumerable<User>> GetDeleted()
  {
      List<User> results = await new Select<User>()
        .Where("Deleted", true)
        .List(_connection);

      return results;
  }

  public async Task<User> Create(CreateUser item)
  {
    UserDB UserDb = new UserDB
    {
      Description = item.Description,
      Name = item.Name,
      FKProfilePicture = item.ProfilePicture?.Id,
    };

    object id = Insert(UserDb, "User");
    var result = await GetById((uint)id);
    return result!;
  }

  public async Task<User> Update(CreateUser item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }
}
