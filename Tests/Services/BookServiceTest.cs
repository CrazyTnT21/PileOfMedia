using Application.Repositories;
using Domain.Classes;
using Domain.Enums;
using Domain.Repositories;
using Domain.Services;
using Domain.ValueObjects;
using Infrastructure.Services;
using Xunit;
using Xunit.Abstractions;

namespace Tests.Services;

public sealed class BookServiceTest
{
  private readonly IBookService _service = new BookService(new BookRepositoryTest(),
                                                           new ImageService(
                                                             new ImageRepositoryTest())); //TODO: Remove later

  private readonly ITestOutputHelper _output;

  public BookServiceTest(ITestOutputHelper output)
  {
    _output = output;
  }

  [Fact]
  public async Task GetNotNull()
  {
    var item = await _service.GetById(1, Language.DA);

    var result = item switch
    {
      Fail<Book?> fail => null,
      Ok<Book?> book => book.Value,
      _ => null
    };
    Assert.False(item.Failure);
    Assert.Equal(1, result?.Id);
  }
}

public sealed class BookRepositoryTest : IBookRepository
{
  public async Task<Book?> GetById(uint id, Language language)
  {
    if (id == 1)
      return new Book()
      {
        Chapters = 25,
        Cover = new Image()
          { Id = 1, Extension = ImageExtension.JPEG, Height = 1920, Width = 1080, Uri = "TestUrl.com" },
        Id = 1,
        Added = new DateOnly(),
        Description = "Description of a test book",
        Favorites = 100,
        Genres = new[] { new Genre() { Id = 1, Name = "Horror" }, new Genre() { Id = 2, Name = "Mystery" } },
        Themes = new[] { new Theme() { Id = 1, Name = "Mythical" }, new Theme() { Id = 2, Name = "Gore" } },
        Members = 10000,
        Pages = 512,
        Popularity = 1,
        Published = new DateOnly(2004, 10, 12),
        Rank = 2,
        Score = (decimal)9.87,
        Title = "Test book",
        Words = 43825
      };
    return null;
  }

  public async Task<IEnumerable<Book>> Get(Language language)
  {
    List<Book> result = new List<Book>();

    for (int i = 0; i < 50; i++)
    {
      result.Add(new Book()
      {
        Chapters = (short)(i * 5),
        Cover = new Image()
          { Id = 1, Extension = ImageExtension.JPEG, Height = 1920, Width = 1080, Uri = "TestUrl.com" },
        Id = i,
        Added = new DateOnly(),
        Description = "Description of a test book",
        Favorites = 100,
        Genres = new[] { new Genre() { Id = 1, Name = "Horror" }, new Genre() { Id = 2, Name = "Mystery" } },
        Themes = new[] { new Theme() { Id = 1, Name = "Mythical" }, new Theme() { Id = 2, Name = "Gore" } },
        Members = 1000 * i,
        Pages = (short)(51 * i + 2),
        Popularity = i,
        Published = new DateOnly(2000 + i, 10, 12),
        Rank = 2 * i,
        Score = (decimal)(new Random().Next(1, 9) + .87),
        Title = "Test book Nr. " + i,
        Words = 438 * i
      });
    }

    return result;
  }

  public async Task<IEnumerable<Book>> GetByTitle(string title, Language language)
  {
    throw new NotImplementedException();
  }

  public async Task<IEnumerable<uint>> GetUserIdsByBook(uint id)
  {
    throw new NotImplementedException();
  }

  public async Task<IEnumerable<User>> GetUsersByBook(uint id)
  {
    throw new NotImplementedException();
  }

  public async Task<Book> Create(CreateBook item)
  {
    throw new NotImplementedException();
  }

  public async Task<Book> Update(CreateBook item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }
}

public sealed class ImageRepositoryTest : IImageRepository
{
  public async Task<Image?> GetById(uint id)
  {
    throw new NotImplementedException();
  }

  public async Task<IEnumerable<Image>> Get()
  {
    throw new NotImplementedException();
  }

  public async Task<Image> Create(CreateImage item)
  {
    throw new NotImplementedException();
  }

  public async Task<Image> Update(CreateImage item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    throw new NotImplementedException();
  }
}
