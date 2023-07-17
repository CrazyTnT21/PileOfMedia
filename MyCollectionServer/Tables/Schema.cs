namespace MyCollectionServer.Tables;


public static class ComicSchema
{
  public const string table = "Comic";
  public static readonly string[] excludeGet = new string[] { fkname, fkdescription, fkstatus,"Name","Description","Status" };
  public static readonly string[] excludeInsert = new string[] { "Name","Description","Status"  };
  public const string pk = "PK";
  public const string fkname = "FKName";
  public const string fkdescription = "FKDescription";
  public const string fkstatus = "FKStatus";
  public const string chapters = "Chapters";
  public const string volumes = "Volumes";
  public const string publishStart = "PublishStart";
  public const string publishEnd = "PublishEnd";
  public const string imageSource = "ImageSource";
  public const string averageScore = "AverageScore";
}
public static class MangaSchema
{
  public const string table = "Manga";
  public static readonly string[] excludeGet = new string[] { fkname, fkdescription, fkstatus };
  public static readonly string[] excludeInsert = new string[] { name, description, status };
  public const string pk = "PK";
  public const string name = "Name";
  public const string fkname = "FKName";
  public const string description = "Description";
  public const string fkdescription = "FKDescription";
  public const string status = "Status";
  public const string fkstatus = "FKStatus";
  public const string chapters = "Chapters";
  public const string volumes = "Volumes";
  public const string publishStart = "PublishStart";
  public const string publishEnd = "PublishEnd";
  public const string imageSource = "ImageSource";
  public const string averageScore = "AverageScore";
}
public static class CharacterSchema
{
  public const string table = "Character";
  public static readonly string[] excludeGet = new string[] { fkname, fkdescription,fklastName,fkfirstName,name, description,lastName,firstName };
  public static readonly string[] excludeInsert = new string[] { name, description,lastName,firstName };
  public const string pk = "PK";
  public const string name = "Name";
  public const string fkname = "FKName";
  public const string firstName = "FirstName";
  public const string fkfirstName = "FKFirstName";
  public const string lastName = "Name";
  public const string fklastName = "FKLastName";
  public const string description = "Description";
  public const string fkdescription = "FKDescription";
  public const string imageSource = "ImageSource";
  public const string birthday = "Birthday";
  public const string height = "Height";
}
public static class TranslationSchema
{
  public const string table = "Translation";
  public const string pk = "PK";
}
public static class AccountSchema
{
  public const string table = "Account";
  public const string fkuser = "FKUser";
  public const string email = "EMail";
  public const string password = "Password";
}
public static class UserSchema
{
  public const string table = "User";
  public const string pk = "PK";
  public const string fkperson = "FKPerson";
  public const string name = "Name";
  public const string joined = "Joined";
  public const string description = "Description";
  public const string imageSource = "ImageSource";
}
public static class GameSchema
{
  public static readonly string[] excludeGet = new string[] { fkname, fkdescription,fkstatus };
  public static readonly string[] excludeInsert = new string[] { name, description,status };

  public const string table = "Game";
  public const string pk = "PK";
  public const string name = "Name";
  public const string fkname = "FKName";
  public const string description = "Description";
  public const string fkdescription = "FKDescription";
  public const string published = "Published";
  public const string averageScore = "AverageScore";
  public const string fkstatus = "FKStatus";
  public const string status = "Status";
  public const string imageSource = "ImageSource";
}
public static class PersonSchema
{
  public static readonly string[] excludeGet = new string[] { fkdescription,description };
  public static readonly string[] excludeInsert = new string[] { description };

  public const string table = "Person";
  public const string pk = "PK";
  public const string name = "Name";
  public const string firstName = "FirstName";
  public const string lastName = "LastName";
  public const string description = "Description";
  public const string fkdescription = "FKDescription";
  public const string birthday = "Birthday";
  public const string height = "Height";
  public const string imageSource = "ImageSource";
}


