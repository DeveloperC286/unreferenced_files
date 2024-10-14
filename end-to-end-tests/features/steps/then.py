from behave import *

from utilities import execute_unreferenced_files
from assertions import *


@then('unreferenced files are not found.')
def assert_unreferenced_files_not_found(context):
    # When
    result = execute_unreferenced_files(context)

    # Then
    assert_no_output(result)
    assert_no_errors(result)
    assert_command_successful(result)
    return result


@then('the unreferenced files are "{unreferenced_files}".')
def assert_unreferenced_files_found(context, unreferenced_files):
    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_unreferenced_files(result, unreferenced_files)


def assert_unreferenced_files_fails(context):
    # When
    result = execute_unreferenced_files(context)

    # Then
    assert_command_unsuccessful(result)
    return result


@then('printed is an error message detailing that the argument search is missing.')
def assert_search_argument_missing_error(context):
    # Given
    search_argument_missing_error = "error: the following required arguments were not provided:\n  --search <SEARCH>\n\nUsage: unreferenced_files --search-for <SEARCH_FOR> --search <SEARCH>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_error_equals(result, search_argument_missing_error)


@then('printed is an error message detailing that the argument search for is missing.')
def assert_search_for_argument_missing_error(context):
    # Given
    search_for_argument_missing_error = "error: the following required arguments were not provided:\n  --search-for <SEARCH_FOR>\n\nUsage: unreferenced_files --search-for <SEARCH_FOR> --search <SEARCH>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_error_equals(result, search_for_argument_missing_error)


@then('printed is an error message detailing that the arguments ignore and only search are mutually exclusive.')
def assert_only_and_ignore_search_mutually_exclusive_error(context):
    # Given
    only_search_mutually_exclusive_error = "error: the argument '--only-search <ONLY_SEARCH>' cannot be used with '--ignore-search <IGNORE_SEARCH>'\n\nUsage: unreferenced_files --search-for <SEARCH_FOR> --search <SEARCH> --only-search <ONLY_SEARCH>\n\nFor more information, try '--help'.\n"
    ignore_search_mutually_exclusive_error = "error: the argument '--ignore-search <IGNORE_SEARCH>' cannot be used with '--only-search <ONLY_SEARCH>'\n\nUsage: unreferenced_files --search-for <SEARCH_FOR> --search <SEARCH> --ignore-search <IGNORE_SEARCH>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_error_is_one_of(result,
                           [only_search_mutually_exclusive_error,
                            ignore_search_mutually_exclusive_error])


@then('printed is an error message detailing that the arguments ignore and only search for are mutually exclusive.')
def assert_only_and_ignore_search_for_mutually_exclusive_error(context):
    # Given
    only_search_for_mutually_exclusive_error = "error: the argument '--only-search-for <ONLY_SEARCH_FOR>' cannot be used with '--ignore-search-for <IGNORE_SEARCH_FOR>'\n\nUsage: unreferenced_files --search-for <SEARCH_FOR> --search <SEARCH> --only-search-for <ONLY_SEARCH_FOR>\n\nFor more information, try '--help'.\n"
    ignore_search_for_mutually_exclusive_error = "error: the argument '--ignore-search-for <IGNORE_SEARCH_FOR>' cannot be used with '--only-search-for <ONLY_SEARCH_FOR>'\n\nUsage: unreferenced_files --search-for <SEARCH_FOR> --search <SEARCH> --ignore-search-for <IGNORE_SEARCH_FOR>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_error_is_one_of(result,
                           [only_search_for_mutually_exclusive_error,
                            ignore_search_for_mutually_exclusive_error])
