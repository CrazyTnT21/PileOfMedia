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
[Route("Users")]
public sealed class UserController : APIController
{
  private readonly IUserService _service;

  public UserController(IUserService service, ILogger logger) : base(logger)
  {
    _service = service;
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<User?>> GetById(uint id)
  {
    return await _service.GetById(id) switch
    {
      Fail<User?> fail => new BadRequestResult(),
      Ok<User?> user => user.Value is null ? new NotFoundResult() : user.Value,
      _ => new StatusCodeResult(500)
    };
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<User>>> Get()
  {
    return (await _service.Get()).ToList();
  }
}
