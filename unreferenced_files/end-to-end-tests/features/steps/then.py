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
    search_argument_missing_error = "error: The following required arguments were not provided:\n" + \
        "    --search <search>...\n" + \
        "\n" + \
        "USAGE:\n" + \
        "    unreferenced_files [FLAGS] [OPTIONS] --search <search>... --search-for <search-for>...\n" + \
        "\n" + \
        "For more information try --help\n"

    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_error_equals(result, search_argument_missing_error)


@then('printed is an error message detailing that the argument search for is missing.')
def assert_search_for_argument_missing_error(context):
    # Given
    search_for_argument_missing_error = "error: The following required arguments were not provided:\n" + \
        "    --search-for <search-for>...\n" + \
        "\n" + \
        "USAGE:\n" + \
        "    unreferenced_files [FLAGS] [OPTIONS] --search <search>... --search-for <search-for>...\n" + \
        "\n" + \
        "For more information try --help\n"

    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_error_equals(result, search_for_argument_missing_error)


@then('printed is an error message detailing that the arguments ignore and only search are mutually exclusive.')
def assert_only_and_ignore_search_mutually_exclusive_error(context):
    # Given
    mutually_exclusive_end = "\n" + \
        "USAGE:\n" + \
        "    unreferenced_files --search <search>... --search-for <search-for>... <--only-search <only-search>...|--ignore-search <ignore-search>...>\n" + \
        "\n" + \
        "For more information try --help\n"

    only_search_mutually_exclusive_error = f"error: The argument '--only-search <only-search>...' cannot be used with one or more of the other specified arguments\n{mutually_exclusive_end}"
    ignore_search_mutually_exclusive_error = f"error: The argument '--ignore-search <ignore-search>...' cannot be used with one or more of the other specified arguments\n{mutually_exclusive_end}"

    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_error_is_one_of(result,
                           [only_search_mutually_exclusive_error,
                            ignore_search_mutually_exclusive_error])


@then('printed is an error message detailing that the arguments ignore and only search for are mutually exclusive.')
def assert_only_and_ignore_search_for_mutually_exclusive_error(context):
    # Given
    mutually_exclusive_end = "\n" + \
        "USAGE:\n" + \
        "    unreferenced_files --search <search>... --search-for <search-for>... <--only-search-for <only-search-for>...|--ignore-search-for <ignore-search-for>...>\n" + \
        "\n" + \
        "For more information try --help\n"

    only_search_for_mutually_exclusive_error = f"error: The argument '--only-search-for <only-search-for>...' cannot be used with one or more of the other specified arguments\n{mutually_exclusive_end}"
    ignore_search_for_mutually_exclusive_error = f"error: The argument '--ignore-search-for <ignore-search-for>...' cannot be used with one or more of the other specified arguments\n{mutually_exclusive_end}"

    # When/Then
    result = assert_unreferenced_files_fails(context)

    # Then
    assert_error_is_one_of(result,
                           [only_search_for_mutually_exclusive_error,
                            ignore_search_for_mutually_exclusive_error])
