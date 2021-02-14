Feature: Unreferenced Files can detect unused and unreferenced files.


  Scenario Outline: Unused and unreferenced files are found.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<search_dir>".
    And the argument --from is provided as "<from_dir>".
    And the flag --assert-no-unreferenced-files is set.
    Then unreferenced files are found.

    Examples:
      | repository                             | checkout_commit                          | search_dir         | from_dir     |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/installation/ | docs/images/ |


  Scenario Outline: Unused and unreferenced files are found.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --search is provided as "<search_dir>".
    And the argument --from is provided as "<from_dir>".
    And the flag --assert-no-unreferenced-files is set.
    Then unreferenced files are not found.

    Examples:
      | repository                                               | checkout_commit                          | search_dir | from_dir     |
      | https://github.com/conventional-changelog/commitlint.git | 6a8b43efc98bcfe46c559fe5c48c6aead3ba4e9a | docs/      | docs/assets/ |
