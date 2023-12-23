using Application.DBMapping;
using Domain.Classes;
using Domain.Classes.DB;
using Domain.Common;
using Domain.Enums;
using Domain.Interfaces;
using Domain.Repositories;
using Domain.ValueObjects;
using Npgsql;

namespace Application.Repositories;

public sealed class BookRepository : Repository, IBookRepository
{
  public BookRepository(NpgsqlConnection connection) : base(connection)
  {
  }

  public async Task<Book?> GetById(uint id, Language language)
  {
    var result = await new Select<Book>()
      .OverrideValue(language, "Language")
      .Where("Id", id)
      .UniqueResult(_connection);

    return result;
  }

  public async Task<IEnumerable<Book>> Get(Language language)
  {
    List<Book> results = await new Select<Book>()
      .OverrideValue(language, "Language")
      .List(_connection);

    return results;
  }

  public async Task<IEnumerable<Book>> GetByTitle(string title, Language language)
  {
    var result = await new Select<Book>()
      .OverrideValue(language, "Language")
      .Where(new TableColumn("Translation", "Title"), title, Comparison.ILike)
      .List(_connection);

    return result;
  }

  public async Task<IEnumerable<User>> GetUsersByBook(uint id)
  {
    List<User> results = await new Select<User>("UserXBook")
      .Join(new Join("User", "User", "UserXBook",
                     new Condition(
                       new TableColumn("UserXBook", "FKUser"),
                       new TableColumn("User", "Id"))))
      .Column(new TableColumn("User", "Id"))
      .List(_connection);
    return results;
  }

  public async Task<IEnumerable<uint>> GetUserIdsByBook(uint id)
  {
    List<uint> results = await new Select<uint>("Book")
      .Projection("Id")
      .List(_connection);
    return results;
  }

  public async Task<Book> Create(CreateBook item)
  {
    BookDB bookDb = new BookDB
    {
      Chapters = item.Chapters,
      Published = item.Published,
      Pages = item.Pages,
      Words = item.Words,
      FKCover = item.Cover!.Id
    };
    var result = await GetById((uint)bookDb.Id, LanguageParser.DEFAULTLANGUAGE);
    return result!;
  }

  public async Task<Book> Update(CreateBook item)
  {
    throw new NotImplementedException();
  }

  public async Task Delete(uint id)
  {
    //Delete From Translation
    //Delete From BookXCharacter
    //Delete From BookXGenre
    //Delete From BookXTheme
    //Delete From BookXCreator
    //Delete From UserXBook
    //Delete
    throw new NotImplementedException();
  }

  public async Task UpdateRank<T>(string table) where T : IEntity, IRank, IScore, new()
  {
    var items = await new Select<T>().OrderBy("Score", Order.Descending).List(_connection);
    for (int i = 0; i < items.Count; i++)
    {
      string update = $"UPDATE ${table} SET RANK = {i} WHERE ID = {items[i].Id}";
    }
  }
}
