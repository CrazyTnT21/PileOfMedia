using System.Text.Json;
using System.Text.Json.Nodes;
using Domain.Classes;
using Domain.Common;
using Domain.Enums;
using Domain.Services;
using Domain.ValueObjects;
using Infrastructure.Services;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;

namespace MyCollectionServer.Controller;

[ApiController]
[Route("Movies")]
public sealed class MovieController : APIController
{
  private readonly IMovieService _service;

  public MovieController(IMovieService service, ILogger logger) : base(logger)
  {
    _service = service;
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<Movie>> GetById(uint id, string? language)
  {
    return await GenericLanguageGetById(id, language, _service);
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<Movie>>> Get(string? language)
  {
    return await GenericLanguageGet(language, _service);
  }

  [HttpPost]
  public async Task<ActionResult<Movie>> Create(CreateMovie composite)
  {
    var timespan = new TimeSpan(5, 12, 34);

    var serialized = JsonSerializer.Serialize(timespan);

    Console.WriteLine(serialized);

    return await _service.Create(composite) switch
    {
      Fail<Movie> fail => new BadRequestResult(),
      Ok<Movie> movie => movie.Value,
      _ => new StatusCodeResult(500)
    };
  }
}
