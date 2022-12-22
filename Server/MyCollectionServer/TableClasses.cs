public class TComic {
    public uint? PK { get; set; }
    public uint FKName { get; set; }
    public uint? FKDescription { get; set; }
    public uint? FKSynopsis { get; set; }
    public uint? Chapters { get;  set; }
    public uint? Volumes { get; set; }
    public DateTime? PublishStart { get; set; }
    public DateTime? PublishEnd { get; set; }
    public string? ImageSource { get; set; }
    public float? AverageScore { get; set; }

    public string? Name { get; set; }
    public string? Description { get; set; }
    public string? Synopsis { get; set; }

    public LanguageField[]? LanguageFields { get; set; }
};
//Other
public class Translation
{
    public string Language { get; set; }
    public string Value { get; set; }
}
public class LanguageField
{
    public Translation[] Values { get; set; }
    public string Column { get; set; }
    public string BindProperty { get; set; }
};


