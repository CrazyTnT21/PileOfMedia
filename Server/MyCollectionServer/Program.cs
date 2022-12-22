using Microsoft.AspNetCore.Cors;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Net.Http.Headers;
using MySqlConnector;
using System.Runtime.CompilerServices;
using System.Text.Json;
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
    options.JsonSerializerOptions.PropertyNamingPolicy = null;
    options.JsonSerializerOptions.DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull;
});
var app = builder.Build();
app.UseHttpsRedirection();
app.UseCors(MyAllowSpecificOrigins);

app.UseAuthorization();
app.MapControllers();

Server.con = new MySqlConnection($"server={args[0]};userid={args[1]};password={args[2]};database={args[3]}");
Server.con.Open();
app.Run();

public class Server
{
    public static MySqlConnection? con;
}

[ApiController]
[Route("[controller]")]
public abstract class BaseClass<T>
{
    [HttpGet]
    public abstract Task<List<T>> GetItems(uint? id);
    [HttpPost]
    public abstract Task CreateItem([FromBody] T item);
    [HttpPut]
    public abstract Task UpdateItem([FromBody] T item);
    [HttpDelete]
    public abstract Task DeleteItem(uint id);
}
