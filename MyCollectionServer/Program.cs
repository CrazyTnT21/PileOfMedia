using System.Text.Json.Serialization;
using Domain.Common;
using Microsoft.Extensions.Logging.Console;
using MyCollectionServer.Core;
using MySqlConnector;

string ip = Environment.GetEnvironmentVariable("IP") ?? throw new Exception($"Enviroment variable 'IP' is missing!");
string user = Environment.GetEnvironmentVariable("USER") ??
              throw new Exception($"Enviroment variable 'USER' is missing!");
string password = Environment.GetEnvironmentVariable("PASSWORD") ??
                  throw new Exception($"Enviroment variable 'PASSWORD' is missing!");
string db = Environment.GetEnvironmentVariable("DB") ?? throw new Exception($"Enviroment variable 'DB' is missing!");

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddCors(options =>
{
  options.AddPolicy("Server", policy => policy.WithOrigins("http://localhost:4000").AllowAnyHeader());
});
using var loggerFactory = LoggerFactory.Create(loggerBuilder =>
{
  loggerBuilder.AddSimpleConsole(i => i.ColorBehavior = LoggerColorBehavior.Enabled);
});
var logger = loggerFactory.CreateLogger<Program>();

builder.Services.AddControllers().ConfigureApiBehaviorOptions(options =>
{
  var builtInFactory = options.InvalidModelStateResponseFactory;
  options.InvalidModelStateResponseFactory = context => builtInFactory(context);
});

builder.Services.AddControllers().AddJsonOptions(options =>
{
  options.JsonSerializerOptions.Converters.Add(new DateOnlyJsonConverter());
  options.JsonSerializerOptions.DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull;
});

var connection = new MySqlConnection($"server={ip};userid={user};password={password};database={db}");

builder.Services.AddSingleton<ILogger>(logger);
builder.Services.AddSingleton(connection);

var app = builder.Build();

app.MapControllers();
app.UseHttpsRedirection()
  .UseCors("Server")
  .UseAuthorization();
app.MapGet("/api/ver", () => "myCollection.ver: 0.01");
app.MapGet("/api/query", BaseT.getParams);
app.MapGet("/api/lang", () => BaseT.languages);

app.Run();
