//#define Benchmark

using System.Text.Json;
using System.Text.Json.Serialization;
using Domain;
using Domain.Common;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Console;
using MyCollectionServer;using MyCollectionServer.Controller;
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

app.Run();
