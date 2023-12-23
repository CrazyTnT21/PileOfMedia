using Application.Repositories;
using Domain.Classes;
using Domain.Enums;

namespace Domain.Repositories;

public interface IBookRepository : IRepository
{
  public Task<Book?> GetById(uint id, Language language);
  public Task<IEnumerable<Book>> Get(Language language);
  public Task<IEnumerable<Book>> GetByTitle(string title, Language language);
  public Task<IEnumerable<uint>> GetUserIdsByBook(uint id);
  public Task<IEnumerable<User>> GetUsersByBook(uint id);
  public Task<Book> Create(CreateBook item);
  public Task<Book> Update(CreateBook item);
  public Task Delete(uint id);
}
