using System.Text.Json.Serialization;
using Application.DBMapping;
using Domain.Common;
using Microsoft.Extensions.Logging.Console;
using MyCollectionServer;
using Npgsql;

string ip = Environment.GetEnvironmentVariable("IP") ?? throw new Exception($"Environment variable 'IP' is missing!");
string user = Environment.GetEnvironmentVariable("USER") ??
              throw new Exception($"Environment variable 'USER' is missing!");
string password = Environment.GetEnvironmentVariable("PASSWORD") ??
                  throw new Exception($"Environment variable 'PASSWORD' is missing!");
string db = Environment.GetEnvironmentVariable("DB") ?? throw new Exception($"Environment variable 'DB' is missing!");

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddCors(options =>
                           options.AddPolicy("Server", policy => policy.AllowAnyOrigin().AllowAnyHeader())
);
using var loggerFactory = LoggerFactory.Create(loggerBuilder =>
                                                 loggerBuilder.AddSimpleConsole(
                                                   i => i.ColorBehavior = LoggerColorBehavior.Enabled)
);
builder.Services.AddControllers().ConfigureApiBehaviorOptions(options =>
{
  var builtInFactory = options.InvalidModelStateResponseFactory;
  options.InvalidModelStateResponseFactory = context => builtInFactory(context);
});

builder.Services.AddControllers().AddJsonOptions(options =>
{
  options.JsonSerializerOptions.Converters.Add(new DateOnlyJsonConverter());
  options.JsonSerializerOptions.DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull;
  options.JsonSerializerOptions.Converters.Add(new JsonStringEnumConverter());
});
DBMapper.SetMappings(true);
builder.DefineImplementations();

var logger = loggerFactory.CreateLogger<Program>();
NpgsqlConnection connection = CreateDBConnection(ip, user, password, db);

builder.Services.AddSingleton<ILogger>(logger);
builder.Services.AddSingleton(connection);

var app = builder.Build();

app.MapControllers();
app.UseHttpsRedirection()
  .UseCors("Server")
  .UseAuthorization();
app.MapGet("/ver", () => "MyCollection.ver: 0.01");
app.Run();



static NpgsqlConnection CreateDBConnection(string ip, string user, string password, string db)
{
  return new NpgsqlConnection($"server={ip};userid={user};password={password};database={db}");
}
