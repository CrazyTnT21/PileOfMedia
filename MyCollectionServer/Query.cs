using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Data.Common;
using System.Reflection;
using System.Text.Json;
using System.Threading.Tasks;
using MySqlConnector;

namespace MyCollectionServer;

public static class QueryBase
{
  public static async Task<List<T>> QueryDB<T>(MySqlCommand cmd, string[]? excludeColumns = null) where T : new()
  {
    // Console.WriteLine(cmd.CommandText);
    List<T> items = new List<T>();
    PropertyInfo[] properties = typeof(T).GetProperties();
    List<int[]> index = new();

    await cmd.PrepareAsync();
    await using MySqlDataReader result = await cmd.ExecuteReaderAsync();

    ReadOnlyCollection<DbColumn> DbColumns = result.GetColumnSchema();
    for (int i = 0; i < properties.Length; i++)
    {
      bool exclude = false;
      for (int k = 0; k < excludeColumns?.Length; k++)
        if (properties[i].GetCustomAttribute<DBColumnAttribute>()?.column == excludeColumns[k])
        {
          exclude = true;
          break;
        }

      if (!exclude)
        for (int j = 0; j < DbColumns.Count; j++)
        {
          DBColumnAttribute? att = properties[i].GetCustomAttribute<DBColumnAttribute>();
          if (att is not null)
            if (att.column.Equals(DbColumns[j].ColumnName, StringComparison.OrdinalIgnoreCase))
            {
              index.Add(new[] { i, j });
              break;
            }
        }
    }

    while (await result.ReadAsync())
    {
      T item = new T();
      for (int i = 0; i < index.Count; i++)
        if (!result.IsDBNull(index[i][1]))
        {
          //TODO: Remove once DateOnly support has been added
          if (properties[index[i][0]].PropertyType == typeof(DateOnly?) ||
              properties[index[i][0]].PropertyType == typeof(DateOnly))
            properties[index[i][0]].SetValue(item, result.GetDateOnly(index[i][1]));
          else
            properties[index[i][0]].SetValue(item, result.GetValue(index[i][1]));
        }

      items.Add(item);
    }

    return items;
  }

  public static async Task<long> QueryDBResult(MySqlCommand cmd)
  {
    await cmd.PrepareAsync();
    await cmd.ExecuteScalarAsync();
    return cmd.LastInsertedId;
  }
}
