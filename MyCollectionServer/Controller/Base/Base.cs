using System.ComponentModel.DataAnnotations.Schema;
using System.Reflection;
using MySqlConnector;

namespace MyCollectionServer.Controller.Base;

public abstract class Base<T> where T : new()
{
  protected readonly MySqlConnection _connection;
  protected readonly ILogger _logger;

  protected Base(ILogger logger, MySqlConnection connection)
  {
    _logger = logger;
    _connection = connection;
  }

  public static List<string> GetColumns() => GetColumns(typeof(T));

  public static List<string> GetColumns<A>() => GetColumns(typeof(A));

  public static List<string> GetColumns(Type type)
  {
    var result = new List<string>();
    var properties = type.GetProperties();
    for (int i = 0; i < properties.Length; i++)
    {
      var column = properties[i].GetCustomAttribute<ColumnAttribute>();
      result.Add(column?.Name ?? properties[i].Name);
    }

    return result;
  }
}
