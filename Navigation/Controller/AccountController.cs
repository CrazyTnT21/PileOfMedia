using Domain.Classes;
using Domain.Services;
using Domain.ValueObjects;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;

namespace MyCollectionServer.Controller;

[ApiController]
[Route("Accounts")]
public sealed class AccountController : APIController
{
  private readonly IAccountService _service;

  public AccountController(IAccountService service, ILogger logger) : base(logger)
  {
    _service = service;
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<Account>> GetById(uint id)
  {
    var result = await _service.GetById(id);

    if (result is Fail<Account> fail)
      return new BadRequestResult();

    Account? account = (Ok<Account?>)result;

    if (account is null)
      return new NotFoundResult();
    return account;
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<Account>>> Get()
  {
    return await _service.Get() switch
    {
      Ok<IEnumerable<Account>> account => account.Value.ToList(),
      Fail<IEnumerable<Account>> fail => new BadRequestResult(),
      _ => new StatusCodeResult(500)
    };
  }

  [HttpPost]
  public async Task<ActionResult<Account>> Create(CreateAccount item)
  {
    return await _service.Create(item) switch
    {
      Ok<Account> account => account.Value,
      Fail<Account> fail => new BadRequestResult(),
      _ => new StatusCodeResult(500)
    };
  }

  [HttpPost]
  public async Task<ActionResult<string>> Login(string username, string password)
  {
    return "";
  }
}
