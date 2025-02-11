from dataclasses import dataclass, field
from typing import List, Set
import unittest

Account = List[str]

@dataclass
class User:
    name: str
    email_addresses: Set[str] = field(default_factory=set)

def account_info(account: Account) -> tuple[str, Set[str]]:
    return (account[0], account[1:])


def accountsMerge(accounts: List[List[str]]) -> List[List[str]]:
    all_users: List[User] = []
    emails_to_users: dict[str, User] = {}
    for account in accounts:
        user, emails = account_info(account)
        matching_user = None
        for email in emails:
            if email in emails_to_users:
                matching_user = emails_to_users[email]
                break
        if matching_user is None:
            matching_user = User(user, set())
            all_users.append(matching_user)
        matching_user.email_addresses.update(emails)
        for email in emails:
            emails_to_users[email] = matching_user
    return [[user.name] + sorted(list(user.email_addresses)) for user in all_users]

class TestAccountsMerge(unittest.TestCase):
    def test_merge_accounts(self):
        accounts = [
            ["John","johnsmith@mail.com","john_newyork@mail.com"],
            ["John","johnsmith@mail.com","john00@mail.com"],
            ["Mary","mary@mail.com"],
            ["John","johnnybravo@mail.com"]
        ]
        expected = [
            ["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],
            ["Mary","mary@mail.com"],
            ["John","johnnybravo@mail.com"]
        ]
        result = accountsMerge(accounts)
        self.assertEqual(sorted(map(tuple, result)), sorted(map(tuple, expected)))

if __name__ == '__main__':
    unittest.main()
