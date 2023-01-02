using Microsoft.AspNetCore.Mvc;
using MySqlConnector;
using System.Collections.ObjectModel;
using System.Data.Common;
using System.Reflection;
using System.Text.Json.Serialization;

var builder = WebApplication.CreateBuilder(args);
var MyAllowSpecificOrigins = "localhost";

builder.Services.AddCors(options =>
{
  options.AddPolicy(name: MyAllowSpecificOrigins,
                    policy =>
                    {
                      policy.WithOrigins("http://localhost:4200").AllowAnyHeader();
                    });
});
builder.Services.AddControllers().ConfigureApiBehaviorOptions(options =>
    {
      var builtInFactory = options.InvalidModelStateResponseFactory;
      options.InvalidModelStateResponseFactory = context =>
      {
        var logger = context.HttpContext.RequestServices.GetRequiredService<ILogger<Program>>();
        return builtInFactory(context);
      };
    });
builder.Services.AddControllers().AddJsonOptions(options =>
{
  // options.JsonSerializerOptions.PropertyNamingPolicy = null;
  options.JsonSerializerOptions.DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull;
});
var app = builder.Build();
app.UseHttpsRedirection();
app.UseCors(MyAllowSpecificOrigins);

app.UseAuthorization();
app.MapControllers();

Server.con = new MySqlConnection($"server={args[0]};userid={args[1]};password={args[2]};database={args[3]}");
Server.con.Open();
Console.WriteLine("Server is listening on: http://localhost:8000");
app.Run();
public class Server { public static MySqlConnection? con; }

[ApiController]
[Route("[controller]")]
public abstract class BaseClass<T> where T : new()
{
  [HttpGet]
  public abstract Task<List<T>> GetItems(uint? id, string? language);
  [HttpPost]
  public abstract Task CreateItem([FromBody] T item);
  [HttpPut]
  public abstract Task UpdateItem([FromBody] T item);
  [HttpDelete]
  public abstract Task DeleteItem(uint id);
  public static async Task<List<T>> QueryDB(string query, string[]? excludeColumns = null)
  {
    Console.WriteLine(query);
    List<T> items = new List<T>();
    var result = await new MySqlCommand(query, Server.con).ExecuteReaderAsync();
    PropertyInfo[] properties = typeof(T).GetProperties();
    List<int[]> index = new();
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
          if (properties[i].GetCustomAttribute<DBColumnAttribute>()?.column == DbColumns[j].ColumnName)
          {
            index.Add(new int[] { i, j });
            break;
          }
    }
    try
    {
      while (await result.ReadAsync())
      {
        T item = new T();
        for (int i = 0; i < index.Count; i++)
          properties[index[i][0]].SetValue(item, !result.IsDBNull(index[i][1]) ? result.GetValue(index[i][1]) : null);
          items.Add(item);
      }
    }
    catch (DbException ex)
    {
      Console.WriteLine(ex);
      Console.WriteLine(ex.ErrorCode);
    }
    finally
    {
      result.Close();
    }
    return items;
  }
}
