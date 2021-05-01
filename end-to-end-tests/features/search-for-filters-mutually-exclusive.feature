Feature: The arguments ignore and only search for are mutually exclusive.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the argument --only-search-for is provided as "<only_search_for>".
    And the argument --ignore-search-for is provided as "<ignore_search_for>".
    Then printed is an error message detailing that the arguments ignore and only search for are mutually exclusive.


    Examples:
      | repository                             | checkout_commit                          | search | search_for               | only_search_for | ignore_search_for |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | ./     | docs/images/main-app.png | "[.]txt$"       | "[.]html$"        |
