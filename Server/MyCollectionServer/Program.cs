//#define Benchmark

using System.Text.Json;
using System.Text.Json.Serialization;
using Microsoft.Extensions.Logging.Console;
using MyCollectionServer;
using MyCollectionServer.Pages;
using MySqlConnector;

#if Benchmark
// await Benchmarker.AsyncBenchmark();
using BenchmarkDotNet.Running;
BenchmarkRunner.Run<Benchmarker>();
return;
#endif

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddCors(options =>
{
  options.AddPolicy(name: "Server",
    policy => { policy.WithOrigins("http://localhost:4200").AllowAnyHeader(); });
});

using var loggerFactory = LoggerFactory.Create(loggerBuilder =>
{
  loggerBuilder.AddSimpleConsole(i => i.ColorBehavior = LoggerColorBehavior.Enabled);
});
var con = new MySqlConnection($"server={args[0]};userid={args[1]};password={args[2]};database={args[3]}");
var logger = loggerFactory.CreateLogger<Program>();
await con.OpenAsync();
builder.Services.AddSingleton<ILogger>(logger);
builder.Services.AddSingleton(con);
builder.Services.AddControllers().ConfigureApiBehaviorOptions(options =>
{
  var builtInFactory = options.InvalidModelStateResponseFactory;
  options.InvalidModelStateResponseFactory = context => builtInFactory(context);
});

builder.Services.AddControllers().AddJsonOptions(options =>
{
  //TODO: Remove once DateOnly support has been added
  options.JsonSerializerOptions.Converters.Add(new DateOnlyJsonConverter());
  options.JsonSerializerOptions.DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull;
});

var app = builder.Build();

app.UseHttpsRedirection();
app.UseCors("Server");
app.UseAuthorization();
app.MapControllers();
// app.Urls.Add("");
app.MapGet("/api/ver", () => "myCollection.ver: 0.01");
app.MapGet("/api/query", BaseT.getParams);
app.MapGet("/api/lang", () => BaseT.languages);
BaseT.languages =
  (await QueryBase.QueryDB<Language>(new MySqlCommand("select Language, `Column` from Language", con))).ToArray();
if (BaseT.languages.Length < 1)
  logger.LogCritical("No Languages have been found, something probably went wrong! Languages: {lang}", BaseT.languages);

// var testuser = new Account();
// testuser.FKUser = 1;
// testuser.Password = "ABCDEFGHJ";
// testuser.Email = "Test@Mail.com";
//  var acc = new AccountClass(logger,con);
// // Console.WriteLine(await acc.verify("ABC"));
//   await acc.CreateItem(testuser);
app.Run();

public sealed class DateOnlyJsonConverter : JsonConverter<DateOnly>
{
  public override DateOnly Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
  {
    return DateOnly.FromDateTime(reader.GetDateTime());
  }

  public override void Write(Utf8JsonWriter writer, DateOnly value, JsonSerializerOptions options)
  {
    writer.WriteStringValue(value.ToString("O"));
  }
}
