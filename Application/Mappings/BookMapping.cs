using Application.DBMapping;
using Domain.Classes;
using Domain.ValueObjects;

namespace Application.Mappings;

public sealed class BookMapping : IMapping
{
  public IDBMapping Mapping { get; } = new DBMapping<Book>()
    .Column(nameof(Book.Id), "Id")
    .Column(nameof(Book.Score), "Score")
    .Column(nameof(Book.Published), "Published")
    .Column(nameof(Book.Added), "Added")
    .Column(nameof(Book.Favorites), "Favorites")
    .Column(nameof(Book.Members), "Members")
    .Column(nameof(Book.Popularity), "Popularity")
    .Column(nameof(Book.Rank), "Rank")
    .Column(nameof(Book.Words), "Words")
    .Column(nameof(Book.Pages), "Pages")
    .Join(nameof(Book.Cover), CoverJoin())
    .Join(TranslationJoin())
    .JoinProjection(nameof(Book.Title), translation, "Title")
    .JoinProjection(nameof(Book.Description), translation, "Description")
    .ArrayJoin(nameof(Book.Genres))
    .ArrayJoin(nameof(Book.Themes));

  private const string book = "Book";
  private const string translation = "Translation";
  private const string bookTranslation = "BookTranslation";

  private static Join TranslationJoin()
  {
    return new Join(bookTranslation, translation, book, new Condition[]
    {
      new(new TableColumn(translation, "Language"), "EN", "Language"),
      new(new TableColumn(translation, "FKTranslation"), new TableColumn(book, "Id"))
    });
  }

  private static Join CoverJoin()
  {
    return new Join("Image", book, new Condition[]
    {
      new(new TableColumn(book, "FKCover"), new TableColumn("Image", "Id"))
    });
  }
}
