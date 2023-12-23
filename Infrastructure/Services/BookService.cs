using Application.Repositories;
using Domain.Classes;
using Domain.Enums;
using Domain.Error;
using Domain.Repositories;
using Domain.Services;
using Domain.ValueObjects;

namespace Infrastructure.Services;

public sealed class BookService : Service, IBookService
{
  private readonly IBookRepository _repository;
  private readonly IImageService _imageService;

  public BookService(IBookRepository repository, IImageService imageService)
  {
    _repository = repository;
    _imageService = imageService;
  }
  public async Task<IResult<Book?>> GetById(uint id, Language language)
  {
    return new Ok<Book?>(await _repository.GetById(id, language));
  }

  public async Task<IResult<IEnumerable<Book>>> GetPreview(uint id, Language language)
  {
    return new Ok<IEnumerable<Book>>(new List<Book>());
    // return await _repository.Get(language);
  }

  public async Task<IResult<IEnumerable<Book>>> Get(Language language)
  {
    var result = await _repository.Get(language);
    return new Ok<IEnumerable<Book>>(result);
  }

  public async Task<IResult<IEnumerable<Book>>> GetByTitle(string title, Language language)
  {
    var books = await _repository.GetByTitle(title, language);
    return new Ok<IEnumerable<Book>>(books);
  }

  public async Task<IResult<Book>> Create(CreateBook item)
  {
    return Validate(item) switch
    {
      Fail fail => new Fail<Book>(fail),
      Ok => await _imageService.Create(item.Cover!) switch
      {
        Fail<Image> fail => new Fail<Book>(fail),
        Ok<Image> image => new Ok<Book>(await Create(item, image)),
        _ => throw new ArgumentException()
      },
      _ => throw new ArgumentException()
    };
  }

  private async Task<Book> Create(CreateBook item, Image image)
  {
    item.Cover!.Id = image.Id;

    return await _repository.Create(item);
  }

  public async Task<IResult<Book>> Update(CreateBook item)
  {
    return new Ok<Book>(await _repository.Update(item));
  }

  public async Task<IResult> Delete(uint id)
  {
    var userCount = (await _repository.GetUserIdsByBook(id)).Count();
    if (userCount > 0)
      return new Fail(new ErrorMessage($"Book has already been added by '{userCount}' users and cannot be deleted"));

    await _repository.Delete(id);
    return new Ok();
  }

  public IResult Validate(CreateBook item)
  {
    if (item.Cover is null)
      return new Fail(new MissingValueErrorMessage("Cover is missing"));

    var result = _imageService.Validate(item.Cover);
    if (result.Failure)
      return result;

    return new Ok();
  }
}
