using Domain.Enums;
using Domain.Error;
using Domain.ValueObjects;

namespace Domain.Common;

public static class LanguageParser
{
  public const Language DEFAULTLANGUAGE = Language.EN;

  public static bool TryGetLanguage(string language, out Language result)
  {
    if (int.TryParse(language, out int number) || !Enum.TryParse(language, true, out Language lang))
    {
      result = DEFAULTLANGUAGE;
      return false;
    }

    result = lang;
    return true;
  }

  public static IResult<Language> GetLanguage(string? language)
  {
    if (language is null)
    {
      return new Ok<Language>(DEFAULTLANGUAGE);
    }

    if (TryGetLanguage(language, out Language languageValue))
    {
      return new Ok<Language>(languageValue);
    }

    return new Fail<Language>(new NotFoundErrorMessage($"Language '{language}' does not exist"));
  }
}
