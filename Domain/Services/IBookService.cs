using Domain.Classes;
using Domain.Common;
using Domain.Enums;
using Domain.Interfaces;
using Domain.ValueObjects;

namespace Domain.Services;

public interface IBookService: ILanguageGetById<Book>,ILanguageGet<Book>
{
  public Task<IResult<IEnumerable<Book>>> GetByTitle(string title, Language language);
  public Task<IResult<Book>> Create(CreateBook item);
  public Task<IResult<Book>> Update(CreateBook item);
  public Task<IResult> Delete(uint id);
  public IResult Validate(CreateBook item);
}
