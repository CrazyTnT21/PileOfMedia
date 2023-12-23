using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Order;
using Domain.Classes;

namespace Benchmarks;

[MemoryDiagnoser]
[Orderer(SummaryOrderPolicy.FastestToSlowest)]
[RankColumn]
public class Benchmarker
{
  [GlobalSetup]
  public void Setup()
  {
    // DBMapper
    //   .AddMapping(new BookMapping().Mapping);
  }

  // [Benchmark]
  // public void SelectAsClass()
  // {
  //   new Select<Book>();
  // }
}
