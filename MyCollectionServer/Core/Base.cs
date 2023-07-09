using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Reflection;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using MySqlConnector;

namespace MyCollectionServer.Core;

public abstract class Base<T>
{
  protected readonly ILogger _logger;
  protected readonly MySqlConnection _mysqlCon;

  public Base(ILogger logger, MySqlConnection mysqlCon)
  {
    _logger = logger;
    _mysqlCon = mysqlCon;
  }

  [NonAction]
  public static List<string> GetColumns(string[]? excludedColumns)
  {
    var att = typeof(T).GetCustomAttribute<DBTableAttribute<T>>();
    if (att is null)
      throw new Exception("Missing DBTableAttribute");

    return excludedColumns is null ? att.columns.ToList() : att.columns.Except(excludedColumns).ToList();
  }

  [NonAction]
  public static List<string> GetColumns<A>(string[]? excludedColumns)
  {
    var att = typeof(A).GetCustomAttribute<DBTableAttribute<A>>();
    if (att is null)
      throw new Exception("Missing DBTableAttribute");

    return excludedColumns is null ? att.columns.ToList() : att.columns.Except(excludedColumns).ToList();
  }

  [NonAction]
  public static List<object?> GetValues(T item, List<string> columns)
  {
    PropertyInfo[] properties = typeof(T).GetProperties();
    List<object?> values = new List<object?>();
    for (int i = 0; i < columns.Count; i++)
    for (int j = 0; j < properties.Length; j++)
      if (columns[i].Equals(properties[j].GetCustomAttribute<DBColumnAttribute>()?.column,
            StringComparison.OrdinalIgnoreCase))
      {
        values.Add(properties[j].GetValue(item));
        break;
      }

    return values;
  }

  [NonAction]
  public async Task<long> Insert(string table, string[] columns, object?[] values)
  {
    if (columns.Length != values.Length)
      throw new Exception("Columns length doesn't match values length");

    await using MySqlCommand cmd =
      new($"INSERT INTO {table}({string.Join(',', columns)}) VALUES({BaseT.RepeatUnique("@v", values.Length)})",
        _mysqlCon);

    BaseT.AddMultipleValues(cmd, BaseT.RepeatUnique("v", values.Length).Split(','), values);
    return await QueryBase.QueryDBResult(cmd);
  }

  //Used for validating whether an item can be inserted/updated
  [NonAction]
  public abstract void Validate(T item, bool update = false);
}

public class HTTPException : Exception
{
  public readonly int StatusCode;
  public readonly string? Reason;

  public HTTPException() : this(StatusCodes.Status500InternalServerError)
  {
  }

  public HTTPException(string reason, Exception? inner = null)
    : this(500, reason, inner)
  {
  }

  public HTTPException(HttpStatusCode statusCode, string? reason = null, Exception? inner = null)
    : this((int)statusCode, reason, inner)
  {
  }

  public HTTPException(int statusCode, string? reason = null, Exception? inner = null)
    : base(reason, inner)
  {
    StatusCode = statusCode;
    Reason = reason;
  }
}
