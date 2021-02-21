Feature: Unreferenced Files can handle multiple files and directories being passed as parameters to search and search for.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<search_file>".
    And the argument --from is provided as "<from_file>".
    And the argument --from is provided as "<from_file_2>".
    And the flag --assert-no-unreferenced-files is set.
    Then the unreferenced files are "<unreferenced_files>".

    Examples:
      | repository                             | checkout_commit                          | search_file    | from_file                | from_file_2                | unreferenced_files                                       |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/readme.md | docs/images/main-app.png | docs/images/screenshot.png | "docs/images/main-app.png\ndocs/images/screenshot.png\n" |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<search_file>".
    And the argument --search is provided as "<search_dir>".
    And the argument --from is provided as "<from_file>".
    And the argument --from is provided as "<from_file_2>".
    And the flag --assert-no-unreferenced-files is set.
    Then unreferenced files are not found.

    Examples:
      | repository                             | checkout_commit                          | search_file | search_dir | from_file                | from_file_2          |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | README.md   | docs/      | docs/images/main-app.png | docs/images/logo.png |
