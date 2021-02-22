Feature: Unreferenced Files can handle multiple files and directories being passed as parameters to search and search for.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<searching_file>".
    And the argument --search-for is provided as "<search_for>".
    And the argument --search-for is provided as "<search_for_2>".
    And the flag --assert-no-unreferenced-files is set.
    Then the unreferenced files are "<unreferenced_files>".
    And the status code is non-zero.

    Examples:
      | repository                             | checkout_commit                          | searching_file | search_for               | search_for_2               | unreferenced_files                                       |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/readme.md | docs/images/main-app.png | docs/images/screenshot.png | "docs/images/main-app.png\ndocs/images/screenshot.png\n" |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<searching_file>".
    And the argument --search is provided as "<searching_dir>".
    And the argument --search-for is provided as "<search_for>".
    And the argument --search-for is provided as "<search_for_2>".
    And the flag --assert-no-unreferenced-files is set.
    Then unreferenced files are not found.

    Examples:
      | repository                             | checkout_commit                          | searching_file | searching_dir | search_for               | search_for_2         |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | README.md      | docs/         | docs/images/main-app.png | docs/images/logo.png |
