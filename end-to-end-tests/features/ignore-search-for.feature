Feature: Ignore and do not search for any files that match any of the provided ignore-search-for regexes.


  Scenario Outline: The singular unreferenced file matches any of the ignore search for regexes, so now there are no unreferenced files.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    And the flag --only-file-name is set.
    Then the unreferenced files are "<unreferenced_files>".
    When the argument --ignore-search-for is provided as "<ignore_search_for>".
    Then unreferenced files are not found.


    Examples:
      | repository                                               | checkout_commit                          | search | search_for   | unreferenced_files              | ignore_search_for |
      | https://github.com/conventional-changelog/commitlint.git | 6a8b43efc98bcfe46c559fe5c48c6aead3ba4e9a | docs/  | docs/assets/ | "docs/assets/commitlint.json\n" | "[.]json$"        |


  Scenario Outline: Some of the unreferenced files match any of the ignore search for regexes, so now there are fewer unreferenced files.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --search is provided as "<search>".
    And the argument --search-for is provided as "<search_for>".
    And the flag --assert-no-unreferenced-files is set.
    Then the unreferenced files are "<unreferenced_files_1>".
    When the argument --ignore-search-for is provided as "<ignore_search_for>".
    Then the unreferenced files are "<unreferenced_files_2>".


    Examples:
      | repository                             | checkout_commit                          | search         | search_for   | unreferenced_files_1                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | ignore_search_for | unreferenced_files_2                |
      | https://github.com/monicahq/monica.git | 88b8ad7af190021bf6dd7f0bfcd1ab76df989d4c | docs/readme.md | docs/images/ | "docs/images/carddav_davx5_1.jpg\ndocs/images/carddav_davx5_1.png\ndocs/images/carddav_token1.png\ndocs/images/carddav_token2.png\ndocs/images/carddav_url.png\ndocs/images/heroku_dashboard-resources.png\ndocs/images/heroku_dashboard.png\ndocs/images/heroku_manage_app.png\ndocs/images/logo.png\ndocs/images/main-app.png\ndocs/images/screenshot.png\ndocs/images/windows10_contacts_1.png\ndocs/images/windows10_contacts_2.png\ndocs/images/windows10_contacts_3.png\ndocs/images/windows10_wheel.png\n" | "[.]png"          | "docs/images/carddav_davx5_1.jpg\n" |
