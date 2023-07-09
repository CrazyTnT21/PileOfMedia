using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using MyCollectionServer.Core;
using MySqlConnector;
using Microsoft.Extensions.Logging;
namespace MyCollectionServer.Pages;

public class GameClass: BaseClass<Game>
{
  public GameClass(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  public override void Validate(Game item, bool update = false)
  {
    throw new NotImplementedException();
  }

  public override async Task<Game?> GetItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override async Task<List<Game>> GetItems(uint? start, uint? limit, string? orderColumn, Order? order)
  {
    throw new NotImplementedException();
  }

  public override async Task<long> CreateItem(Game item)
  {
    throw new NotImplementedException();
  }

  public override async Task UpdateItem(Game item)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItems(uint[] id)
  {
    throw new NotImplementedException();
  }
}
