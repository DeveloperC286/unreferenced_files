Feature: Unreferenced Files can handle file being passed as parameters to search and search for.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<searching>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    Then the unreferenced files are "<unreferenced_files>".
    And the status code is non-zero.

    Examples:
      | repository                             | checkout_commit                          | searching      | search_for               | unreferenced_files           |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/readme.md | docs/images/main-app.png | "docs/images/main-app.png\n" |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<searching>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    Then unreferenced files are not found.

    Examples:
      | repository                             | checkout_commit                          | searching | search_for               |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | README.md | docs/images/main-app.png |
