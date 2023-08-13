using Application.Controller;
using Domain;
using Domain.Classes;
using Domain.Enums;
using Microsoft.AspNetCore.Mvc;
using MyCollectionServer.Controller.Base;
using MyCollectionServer.Miscellaneous;
using MySqlConnector;

namespace MyCollectionServer.Controller;

[ApiController]
[Route("Books")]
public class BookController : APIController<Book, BookValidateOption>, IBookService
{
  public BookController(ILogger logger, MySqlConnection connection) : base(logger, connection)
  {
  }

  [HttpGet("{id}")]
  public async Task<ActionResult<Book?>> GetSingleResult(uint id, string? language)
  {
    var options = new BookGetOption(language ?? "EN");
    Book? result = await GetSingle(id, options);
    if (result is null)
      return new NotFoundResult();
    return result;
  }

  [NonAction]
  public async Task<Book?> GetSingle(uint id, BookGetOption options)
  {
    await _connection.OpenAsync();
    List<Book> results = await new Select<Book>()
      .Take(1)
      .AddDynamicColumn(Key.Language, "EN")
      .Where("B.Id", id)
      .QueryDB(_connection).ToList();
    await _connection.CloseAsync();

    if (results.Count == 1)
      return results[0];
    return null;
  }

  [HttpGet]
  public async Task<ActionResult<IEnumerable<Book>>> GetResult(string? language)
  {
    var options = new BookGetOption(language ?? "EN");
    IEnumerable<Book> result = await Get(options);
    return result.ToList();
  }

  [NonAction]
  public async Task<IEnumerable<Book>> Get(BookGetOption options)
  {
    await _connection.OpenAsync();
    List<Book> results = await new Select<Book>()
      .AddDynamicColumn(Key.Language, options.Language)
      .QueryDB(_connection)
      .ToList();
    await _connection.CloseAsync();

    return results;
  }

  [NonAction]
  public async Task<Book?> Create(Book item, BookCreateOption options)
  {
    throw new NotImplementedException();
  }

  [NonAction]
  public async Task<Book> Update(Book item, BookUpdateOption options)
  {
    throw new NotImplementedException();
  }

  [NonAction]
  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }

  [NonAction]
  public async Task Delete(uint[] ids)
  {
    throw new NotImplementedException();
  }

  [NonAction]
  public override HTTPError? Validate(Book item, BookValidateOption options)
  {
    throw new NotImplementedException();
  }
}

public struct BookValidateOption
{
  public bool Update;
  public LanguageField[]? LanguageFields;
}
