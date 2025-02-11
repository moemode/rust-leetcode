from dataclasses import dataclass, field
from typing import List, Set, FrozenSet
import unittest

Account = List[str]

@dataclass(frozen=True)
class User:
    name: str
    email_addresses: FrozenSet[str] = field(default_factory=frozenset)

def account_info(account: Account) -> tuple[str, FrozenSet[str]]:
    return (account[0], frozenset(account[1:]))

def consolidateUsers(newUser: User, matchingUsers: List[User]) -> User:
    all_emails = set(newUser.email_addresses)
    for user in matchingUsers:
        all_emails.update(user.email_addresses)
    return User(newUser.name, frozenset(all_emails))

def accountsMerge(accounts: List[List[str]]) -> List[List[str]]:
    all_users: Set[User] = set()
    emails_to_users: dict[str, User] = {}
    for account in accounts:
        name, emails = account_info(account)
        matching_users = []
        for email in emails:
            if email in emails_to_users:
                matching_users.append(emails_to_users[email])
        all_users.difference_update(matching_users)
        consolidated = consolidateUsers(User(name, emails), matching_users)
        all_users.add(consolidated)
        for email in consolidated.email_addresses:
            emails_to_users[email] = consolidated
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
