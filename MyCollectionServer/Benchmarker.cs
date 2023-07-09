using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Order;
using MyCollectionServer.Pages;
using MySqlConnector;

namespace MyCollectionServer;

[MemoryDiagnoser]
[Orderer(SummaryOrderPolicy.FastestToSlowest)]
[RankColumn]
public class Benchmarker
{
  private readonly Join[] testjoins = new Join[]
  {
    // new("Translation", "EN", "Name", "Comic", "FKName", "PK", "Comic", "PK", 2, JoinType.Inner),
    // new("Translation", "EN", "Description", "Comic", "FKDescription", "PK"),
    // new("ComicXCreator", "FKPerson", "Creator", "Comic", "PK", "FKComic"),
  };

  private static Comic tesComic = new Comic()
  {
    LanguageFields = new LanguageField[]
    {
      new("Name", "fkName", new Translation("TestEnglish", "EN")),
      new("Name", "fkName", new Translation("TestEnglish", "EN")),
      new("Name", "fkName", new Translation("TestEnglish", "EN")),
      new("Name", "fkName", new Translation("TestEnqweglish", "EN")),
      new("Name", "fkName", new Translation("TestEnaweqeqewqeqweglish", "EN")),
      new("Name", "fkName", new Translation("TestEaweawenglish", "EN")),
      new("Name", "fkName", new Translation("TestEnawewaeaweglish", "EN")),
      new("Name", "fkName", new Translation("TestEnaweaweaweglish", "EN")),
      new("Name", "fkName", new Translation("TestEngaweaweaweawelish", "EN")),
      new("Name", "fkName", new Translation("TestEneaweaweaglish", "EN")),
      new("Name", "fkName", new Translation("   ", "EN")),
      new("Name", "fkName", new Translation(" ", "EN")),
      new("Name", "fkName", new Translation("           ", "EN")),
      new("Name", "fkName", new Translation("", "EN")),
      new("Name", "fkName", new Translation("         ", "EN")),
      new("Name", "fkName", new Translation("                      ", "EN"))
    }
  };

  public static async Task AsyncBenchmark()
  {
  //
  //   await con.OpenAsync();
  //
  //   var watch = new Stopwatch();
  //   var item = new ComicClass(null, con);
  //   List<Comic> items = new List<Comic>();
  //   var manga = new MangaClass(null, con);
  //   List<Manga> mangas = new List<Manga>();
  //   watch.Start();
  //   for (int i = 0; i < 10; i++)
  //   {
  //     var better = new Stopwatch();
  //     better.Start();
  //     items.AddRange(await item.GetItems("EN"));
  //     mangas.AddRange(await manga.GetItems("EN", null, null, null, null));
  //     better.Stop();
  //     Console.WriteLine("elapsed: " + better.ElapsedMilliseconds);
  //   }
  //
  //   watch.Stop();
  //   Console.WriteLine(watch.ElapsedMilliseconds);
  //   await con.CloseAsync();
  }

  [Benchmark]
  public void IterateListConcate()
  {
    List<string> testList = new List<string>()
    {
      "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test"
    };
    for (int i = 0; i < testList.Count; i++)
      testList[i] += testList[i];
  }

  [Benchmark]
  public void IterateArrayConcate()
  {
    string[] testArray =
    {
      "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test"
    };
    for (int i = 0; i < testArray.Length; i++)
      testArray[i] += testArray[i];
  }

  [Benchmark]
  public void IterateList()
  {
    List<string> testList = new List<string>()
    {
      "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test"
    };
    for (int i = 0; i < testList.Count; i++)
    {
    }
  }

  [Benchmark]
  public void IterateArray()
  {
    string[] testArray =
    {
      "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test", "Test"
    };
    for (int i = 0; i < testArray.Length; i++)
    {
    }
  }

  [Benchmark]
  public void SelectLeftJoin()
  {
    BaseT.SelectLeftJoin("Comic", testjoins, 3, 60);
  }

  [Benchmark]
  public void Limit()
  {
    BaseT.Limit(2, 25);
  }

  [Benchmark]
  public void IsOnlyWhiteSpace()
  {
    BaseT.IsOnlyWhiteSpace(tesComic.LanguageFields[0].Values);
  }

  [Benchmark]
  public void RepeatUnique()
  {
    BaseT.RepeatUnique("@TESTVALUE", 10);
  }

  [Benchmark]
  public void IsValidWhiteSpace()
  {
    BaseT.IsValidWhiteSpace("TName", tesComic.LanguageFields);
  }

  [Benchmark]
  public void IsValidColumn()
  {
    BaseT.IsValidColumn("ImageSource", tesComic.LanguageFields);
  }
}
