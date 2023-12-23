using System.Security.Cryptography;
using Application.DBMapping;
using Domain.Classes;
using Domain.Classes.DB;
using Domain.Error;
using Domain.ValueObjects;
using Npgsql;

namespace Application.Repositories;

public sealed class AccountRepository : Repository, IAccountRepository
{
  public AccountRepository(NpgsqlConnection connection) : base(connection)
  {
  }

  public async Task<Account?> GetById(uint id)
  {
    var result = await new Select<Account>()
      .Where("Id", id)
      .UniqueResult(_connection);

    return result;
  }

  public async Task<IEnumerable<Account>> Get()
  {
    return await new Select<Account>().List(_connection);
  }

  public async Task<Account> Create(CreateAccount item)
  {
    AccountDB AccountDb = new AccountDB
    {
      FKUser = item.User.Id,
      Password = CreateHashedPassword(item.Password),
      Email = item.Email
    };
    object id = Insert(AccountDb, "Account");
    var result = await GetById((uint)id);
    return result!;
  }

  private string CreateHashedPassword(string password)
  {
    byte[] salt = RandomNumberGenerator.GetBytes(16);
    var pbkdf2 = new Rfc2898DeriveBytes(password, salt, 100000);
    byte[] hash = pbkdf2.GetBytes(20);
    byte[] hashBytes = new byte[36];
    Array.Copy(salt, 0, hashBytes, 0, 16);
    Array.Copy(hash, 0, hashBytes, 16, 20);
    return Convert.ToBase64String(hashBytes);
  }

  public IResult Verify(Account account, string password)
  {
    byte[] hashBytes = Convert.FromBase64String(account.Password);
    byte[] salt = new byte[16];
    Array.Copy(hashBytes, 0, salt, 0, 16);

    var pbkdf2 = new Rfc2898DeriveBytes(password, salt, 100000);
    byte[] hash = pbkdf2.GetBytes(20);

    /* Compare the results */
    for (int i = 0; i < 20; i++)
      if (hashBytes[i + 16] != hash[i])
        return new Fail(new ErrorMessage("Wrong username or password"));
    return new Ok();
  }

  public async Task<Account> Update(CreateAccount item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }
}
