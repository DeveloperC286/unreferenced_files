Feature: Only search files that match any of the provided only-search regexes.


  Scenario Outline: The only reference for the singular file being searched for does not match any of the only search regexes, so it is now an unreferenced files.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    Then unreferenced files are not found.
    When the argument --only-search is provided as "<only_search>".
    Then the unreferenced files are "<unreferenced_files>".


    Examples:
      | repository                             | checkout_commit                          | search | search_for               | only_search | unreferenced_files           |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | ./     | docs/images/main-app.png | "[.]txt$"   | "docs/images/main-app.png\n" |


  Scenario Outline: One of multiple references for the singular file being searched for does not match any of the only search regexes, so there are still no unreferenced files.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    And the flag --only-file-name is set.
    Then unreferenced files are not found.
    When the argument --only-search is provided as "<only_search>".
    Then unreferenced files are not found.


    Examples:
      | repository                                               | checkout_commit                          | search | search_for                 | only_search         |
      | https://github.com/conventional-changelog/commitlint.git | 6a8b43efc98bcfe46c559fe5c48c6aead3ba4e9a | ./     | docs/assets/commitlint.svg | "docs/README[.]md$" |


  Scenario Outline: The only reference for the multiple file being searched for does not match any of the only search regexes, so they are now unreferenced files. However, not all searched for files are unreferenced.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    And the flag --only-file-name is set.
    Then the unreferenced files are "<unreferenced_files_1>".
    When the argument --only-search is provided as "<only_search>".
    Then the unreferenced files are "<unreferenced_files_2>".


    Examples:
      | repository                                               | checkout_commit                          | search | search_for   | unreferenced_files_1            | only_search | unreferenced_files_2                                                        |
      | https://github.com/conventional-changelog/commitlint.git | 6a8b43efc98bcfe46c559fe5c48c6aead3ba4e9a | docs/  | docs/assets/ | "docs/assets/commitlint.json\n" | "[.]md$"    | "docs/assets/commitlint.json\ndocs/assets/icon.png\ndocs/assets/icon.svg\n" |
