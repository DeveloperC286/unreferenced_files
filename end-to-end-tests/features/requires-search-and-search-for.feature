Feature: Unreferenced Files requires the parameters --search and --search-for.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search-for is provided as "<search_for>".
    Then the status code is non-zero.

    Examples:
      | repository                             | checkout_commit                          | search_for   |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/images/ |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<searching>".
    Then the status code is non-zero.

    Examples:
      | repository                                               | checkout_commit                          | searching |
      | https://github.com/conventional-changelog/commitlint.git | 6a8b43efc98bcfe46c559fe5c48c6aead3ba4e9a | docs/     |
