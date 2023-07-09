using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using MyCollectionServer.Core;
using MySqlConnector;
using Microsoft.Extensions.Logging;
namespace MyCollectionServer;

public sealed class TranslationClass: BaseClass<Language>
{
  public static string GetLanguage(string? language)
  {
    return (language is null || !Array.Exists(BaseT.languages!, x => x.ColumnName == language)) ? "EN" : language;
  }
  public static async Task UpdateItem(uint id)
  {
    throw new NotImplementedException();
  }

  public static async Task DeleteItem(params uint[] id)
  {
    throw new NotImplementedException();
  }

  public async Task<long?> CreateItem(string column, LanguageField[] items)
  {
    for (int i = 0; i < items.Length; i++)
      if (column.Equals(items[i].Column,StringComparison.OrdinalIgnoreCase))
      {
        if (ValidateEmpty(items[i].Values))
          return null;
        if (!Validate(items[i].Values))
        {
          throw new Exception("Not Valid");
        }
        string[] columns = new string[items[i].Values.Length];
        object?[] values = new object?[items[i].Values.Length];
        for (int j = 0; j < items[i].Values.Length; j++)
        {
          columns[j] = items[i].Values[j].Language;
          values[j] = items[i].Values[j].Value;
        }
        return await Insert("Translation", columns, values);
      }
    return null;
  }

  private static bool Validate(Translation[] item)
  {
    for (int i = 0; i < item.Length; i++)
    {
      if (item[i].Value.Length > 500 || string.IsNullOrWhiteSpace(item[i].Language))
        return false;
    }

    return true;
  }

  private static bool ValidateEmpty(Translation[] item)
  {
    for (int i = 0; i < item.Length; i++)
    {
      if (string.IsNullOrWhiteSpace(item[i].Value))
        return true;
    }

    return false;
  }

  public TranslationClass(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  public async Task<Language?> GetItem(uint id, string language)
  {
    throw new NotImplementedException();
  }

  public override async Task<Language?> GetItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override Task<List<Language>> GetItems(uint? start, uint? limit, string? orderColumn, Order? order)
  {
    throw new NotImplementedException();
  }

  public override Task<long> CreateItem(Language item)
  {
    throw new NotImplementedException();
  }

  public override Task UpdateItem(Language item)
  {
    throw new NotImplementedException();
  }

  public override Task DeleteItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItems(uint[] id)
  {
    throw new NotImplementedException();
  }

  public override void Validate(Language item, bool update = false)
  {
    throw new NotImplementedException();
  }
}
