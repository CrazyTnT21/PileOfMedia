using System;
using System.Collections.Generic;
using System.Net;
using System.Security.Cryptography;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Core;
using MySqlConnector;
using Microsoft.Extensions.Logging;

namespace MyCollectionServer.Pages;

public class AccountClass : BaseClassSingle<Account>
{
  public AccountClass(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  [HttpGet]
  public async Task<Account?> GetItem(string password, string email)
  {
    List<Account> users = await QueryBase.QueryDB<Account>(new MySqlCommand(
      $"select * from Account where EMail = {email}", _mysqlCon));

    return null;
  }

  [NonAction]
  public override async Task<Account?> GetItem(uint id, string language)
  {
    throw new NotImplementedException();
  }

  public async override Task<long> CreateItem(Account item)
  {
    byte[] salt = RandomNumberGenerator.GetBytes(16);
    var pbkdf2 = new Rfc2898DeriveBytes(item.Password, salt, 100000);
    byte[] hash = pbkdf2.GetBytes(20);
    byte[] hashBytes = new byte[36];
    Array.Copy(salt, 0, hashBytes, 0, 16);
    Array.Copy(hash, 0, hashBytes, 16, 20);
    string savedPasswordHash = Convert.ToBase64String(hashBytes);
    item.Password = savedPasswordHash;
    List<string> columns = GetColumns(null);
    List<object?> values = GetValues(item, columns);
    return await Insert(AccountSchema.table, columns.ToArray(), values.ToArray());
  }

  public override Task UpdateItem(Account item)
  {
    throw new NotImplementedException();
  }

  public override Task DeleteItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override void Validate(Account item, bool update = false)
  {
    /* Fetch the stored value */
    /* Extract the bytes */
    string savedPasswordHash = item.Password;
    byte[] hashBytes = Convert.FromBase64String(savedPasswordHash);
    /* Get the salt */
    byte[] salt = new byte[16];
    Array.Copy(hashBytes, 0, salt, 0, 16);
    /* Compute the hash on the password the user entered */
    var pbkdf2 = new Rfc2898DeriveBytes(item.Password, salt, 100000);
    byte[] hash = pbkdf2.GetBytes(20);
    /* Compare the results */
    for (int i = 0; i < 20; i++)
      if (hashBytes[i + 16] != hash[i])
        throw new HTTPException(StatusCodes.Status400BadRequest, "Wrong username or password");
  }
}
