Feature: Unreferenced Files can handle files being passed as parameters to search and search for.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    Then the unreferenced files are "<unreferenced_files>".

    Examples:
      | repository                             | checkout_commit                          | search         | search_for               | unreferenced_files           |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/readme.md | docs/images/main-app.png | "docs/images/main-app.png\n" |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    Then unreferenced files are not found.

    Examples:
      | repository                             | checkout_commit                          | search    | search_for               |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | README.md | docs/images/main-app.png |
