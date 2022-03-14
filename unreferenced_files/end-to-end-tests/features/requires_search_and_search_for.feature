Feature: Unreferenced Files requires the parameters --search and --search-for.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search-for is provided as "<search_for>".
    Then printed is an error message detailing that the argument search is missing.

    Examples:
      | repository                             | checkout_commit                          | search_for   |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/images/ |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    Then printed is an error message detailing that the argument search for is missing.

    Examples:
      | repository                                               | checkout_commit                          | search |
      | https://github.com/conventional-changelog/commitlint.git | 6a8b43efc98bcfe46c559fe5c48c6aead3ba4e9a | docs/  |
