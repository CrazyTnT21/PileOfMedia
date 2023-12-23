using System.Text;
using Npgsql;

namespace Application.Repositories;

public class Repository
{
  protected NpgsqlConnection _connection;

  public Repository(NpgsqlConnection connection)
  {
    _connection = connection;
  }

  public async Task<object?> Insert<T>(T item, string table)
  {
    var properties = typeof(T).GetProperties();
    List<string> columns = new List<string>(properties.Length);
    List<object?> values = new List<object?>(properties.Length);

    for (int i = 0; i < properties.Length; i++)
    {
      var value = properties[i].GetValue(item);
      if (value is null)
        continue;
      columns.Add(properties[i].Name);
      values.Add(value);
    }

    var query = CreateQuery(table, columns.ToArray(), values.ToArray());
    var cmd = new NpgsqlCommand(query, _connection);
    return await cmd.ExecuteScalarAsync();
  }

  protected string CreateQuery(string table, string[] columns, IReadOnlyList<object?> values)
  {
    if (columns.Length == 0)
      return $"INSERT INTO {table} default values";

    var result = new StringBuilder($"INSERT INTO {table} (${string.Join(',', columns)})");
    result.Append("VALUES (");
    AddValue(result, values[0]);
    for (int i = 1; i < values.Count; i++)
    {
      result.Append(',');
      AddValue(result, values[i]);
    }

    result.Append(");");
    return result.ToString();
  }

  private void AddValue(StringBuilder builder, object? value)
  {
    if (value is null)
    {
      builder.Append("NULL");
      return;
    }

    if (value is string)
    {
      builder.Append($"\"{value}\"");
      return;
    }

    builder.Append(value);
  }
}
