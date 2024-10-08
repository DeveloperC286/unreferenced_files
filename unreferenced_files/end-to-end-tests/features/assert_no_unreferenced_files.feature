Feature: When using the --assert-no-unreferenced-files flag if unreferenced files are found then a nonzero status code is returned.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    Then the unreferenced files are "<unreferenced_files>".

    Examples:
      | repository                             | checkout_commit                          | search             | search_for   | unreferenced_files                                                                                                                                                                                                                                                                                                                                                         |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/installation/ | docs/images/ | "docs/images/carddav_davx5_1.jpg\ndocs/images/carddav_davx5_1.png\ndocs/images/carddav_token1.png\ndocs/images/carddav_token2.png\ndocs/images/carddav_url.png\ndocs/images/main-app.png\ndocs/images/screenshot.png\ndocs/images/windows10_contacts_1.png\ndocs/images/windows10_contacts_2.png\ndocs/images/windows10_contacts_3.png\ndocs/images/windows10_wheel.png\n" |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    Then unreferenced files are not found.

    Examples:
      | repository                                               | checkout_commit                          | search | search_for   |
      | https://github.com/conventional-changelog/commitlint.git | 6a8b43efc98bcfe46c559fe5c48c6aead3ba4e9a | docs/  | docs/assets/ |
