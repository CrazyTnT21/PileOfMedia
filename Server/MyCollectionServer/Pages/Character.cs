using MyCollectionServer.Core;
using MySqlConnector;

namespace MyCollectionServer.Pages;

public class CharacterClass : BaseClass<Character>
{
  public CharacterClass(ILogger logger, MySqlConnection mysqlCon) : base(logger, mysqlCon)
  {
  }

  public override void Validate(Character item, bool update = false)
  {
    throw new NotImplementedException();
  }

  public override async Task<Character?> GetItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override async Task<List<Character>> GetItems(uint? start, uint? limit, string? orderColumn, Order? order)
  {
    throw new NotImplementedException();
  }

  public override async Task<long> CreateItem(Character item)
  {
    throw new NotImplementedException();
  }

  public override async Task UpdateItem(Character item)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItem(uint id)
  {
    throw new NotImplementedException();
  }

  public override async Task DeleteItems(uint[] id)
  {
    throw new NotImplementedException();
  }

  public static Select GetX(Select select, string language)
  {
    return select
      .AddColumns(CharacterSchema.table, GetColumns<Character>(CharacterSchema.excludeGet))
      .AddColumn(language, "TFirstName", CharacterSchema.firstName)
      .AddColumn(language, "TName", "Name")
      .AddColumn(language, "TDescription", "Description")
      .Join(new Join(TranslationSchema.table, TranslationSchema.pk, CharacterSchema.table, CharacterSchema.fkfirstName, "TFirstName"))
      .Join(new Join(TranslationSchema.table, TranslationSchema.pk, CharacterSchema.table, CharacterSchema.fkname, "TName"))
      .Join(new Join(TranslationSchema.table, TranslationSchema.pk, CharacterSchema.table, CharacterSchema.fkdescription, "TDescription"));
  }
}
