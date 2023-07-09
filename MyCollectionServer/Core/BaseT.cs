using System;
using System.Text;
using Microsoft.Extensions.Logging;
using MySqlConnector;

namespace MyCollectionServer;

public static class BaseT
{
  public static Language[]? languages;
  private const uint count = 50;

  public static string getParams()
  {
    StringBuilder Params = new("");
    for (int i = 0; i < queryParams.GetLength(0); i++)
    {
      for (int j = 0; j < queryParams.GetLength(1); j++)
        Params.Append(queryParams[i, j] + " ");
      Params.Append("\n");
    }

    return Params.ToString();
  }

  private static readonly string[,] queryParams =
  {
    { "LANG", "Language" },
    { "SORT", "Sort" },
    { "TEXT", "Search text" },
    { "SECO", "Search column" },
    { "ORDER", "Order by" },
    { "POS", "Start position" },
    { "AMNT", "Amount of items (max. 50)" },
  };

  public static string SelectLeftJoin(string table, Join[] joins, uint? start = 0, uint? count = count)
  {
    StringBuilder query = new($"SELECT {table}.*");
    for (int i = 0; i < joins.Length; i++)
      query.Append($",{joins[i].Alias}. AS {joins[i].Alias}");

    query.Append($" FROM {table} ");
    for (int i = 0; i < joins.Length; i++)
      query.Append(joins[i].createJoin());
    query.Append(Limit(start, count));
    return query.ToString();
  }

  public static string Select(string table, string[]? columns = null, string? whereColumn = null,
    object? whereValue = null, uint? start = 0,
    uint? count = count)
  {
    StringBuilder query = new StringBuilder("SELECT ");
    if (columns is null)
      query.Append("*");
    else
      query.Append(string.Join(',', columns));
    query.Append($" FROM {table}");
    if (whereValue is not null && whereColumn is not null)
      query.Append($" WHERE {whereColumn} = {whereValue}");
    query.Append(Limit(start, count));
    return query.ToString();
  }

  public static string Limit(uint? start = 0, uint? _count = count)
  {
    start ??= 0;

    if (_count is null || _count > 50)
      _count = count;
    //Nullable types are 3x slower
    return $" LIMIT {(uint)start},{(uint)_count}";
  }

  public static string OrderBy(string column, Order order = Order.Ascending)
  {
    return $" ORDER BY {column} {(order == Order.Ascending ? string.Empty : "DESC")}";
  }

  public static string OrderByWithout(string column, Order order = Order.Ascending)
  {
    return $"{column} {(order == Order.Ascending ? string.Empty : "DESC")}";
  }

  //Checks if the column exists and does not just contain whitespace
  public static bool IsValidColumn(string column, LanguageField[]? languageFields)
  {
    if (languageFields is null)
      return false;

    bool missingValue = true;

    for (int i = 0; i < languageFields.Length; i++)
      if (column.Equals(languageFields[i].Column, StringComparison.OrdinalIgnoreCase))
      {
        if (IsOnlyWhiteSpace(languageFields[i].Values))
          return false;
        missingValue = false;
      }

    return !missingValue;
  }

  //Checks if the translation values contain whitespace only values
  public static bool IsOnlyWhiteSpace(Translation[]? values)
  {
    if (values is null)
      return true;
    for (int j = 0; j < values.Length; j++)
      if (!string.IsNullOrWhiteSpace(values[j].Value))
        return false;
    return true;
  }

  //True if the column doesn't exist or column exists with a value
  public static bool IsValidWhiteSpace(string column, LanguageField[]? languageFields)
  {
    if (languageFields is null)
      return false;

    for (int i = 0; i < languageFields.Length; i++)
      if (languageFields[i].Column == column)
        return !IsOnlyWhiteSpace(languageFields[i].Values);

    return true;
  }

  public static void AddMultipleValues(MySqlCommand cmd, string[] names, object[] values)
  {
    if (names.Length != values.Length)
      throw new Exception(
        $"Names.Length does not match values.Length {names.Length}({string.Join(',', names)}) != {values.Length}({string.Join(',', values)})");
    for (int i = 0; i < names.Length; i++)
    {
      cmd.Parameters.AddWithValue(names[i], values[i]);
    }
  }

  public static string RepeatUnique(string value, int amount)
  {
    StringBuilder result = new StringBuilder(value + '0', amount * value.Length + amount - 2);
    for (int i = 1; i < amount; i++)
    {
      result.Append(',');
      result.Append(value);
      result.Append(i);
    }
    return result.ToString();
  }

  public static void LogWarning(ILogger logger, Exception exception)
  {
    logger.LogWarning("{time}: {ex}", DateTime.Now, exception);
  }

  public static void LogError(ILogger logger, Exception exception)
  {
    logger.LogError("{time}: {ex}", DateTime.Now, exception);
  }
}
