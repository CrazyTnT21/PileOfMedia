namespace Domain.Classes;

public sealed class Account
{
    public User User { get; set; } = null!;
    public string Email { get; set; } = null!;
    public string Password { get; set; } = null!;
}
public sealed class CreateAccount
{
    public CreateUser User { get; set; } = null!;
    public string Email { get; set; } = null!;
    public string Password { get; set; } = null!;
}
