from behave import *

from utilities import execute_unreferenced_files


@then('unreferenced files are not found.')
def then_unreferenced_files_not_found(context):
    # When
    execute_unreferenced_files(context)

    # Then
    assert context.stdout == ""
    assert context.stderr == ""
    assert int(context.exit_code) == 0


@then('the unreferenced files are "{unreferenced_files}".')
def then_unreferenced_files_found(context, unreferenced_files):
    # When
    execute_unreferenced_files(context)

    # Then
    unreferenced_files = unreferenced_files.strip() \
        .strip('\"').replace("\\n", '\n')
    assert context.stdout == unreferenced_files
    assert int(context.exit_code) != 0


def then_nonzero_status_code(context):
    # When
    execute_unreferenced_files(context)

    # Then
    assert int(context.exit_code) != 0


@then('printed is an error message detailing that the argument search is missing.')
def then_search_argument_missing_error(context):
    # Given
    search_argument_missing_error = "error: The following required arguments were not provided:\n" + \
        "    --search <search>...\n" + \
        "\n" + \
        "USAGE:\n" + \
        "    unreferenced_files [FLAGS] [OPTIONS] --search <search>... --search-for <search-for>...\n" + \
        "\n" + \
        "For more information try --help\n"

    # When/Then
    then_nonzero_status_code(context)

    # Then
    assert context.stderr == search_argument_missing_error


@then('printed is an error message detailing that the argument search for is missing.')
def then_search_for_argument_missing_error(context):
    # Given
    search_for_argument_missing_error = "error: The following required arguments were not provided:\n" + \
        "    --search-for <search-for>...\n" + \
        "\n" + \
        "USAGE:\n" + \
        "    unreferenced_files [FLAGS] [OPTIONS] --search <search>... --search-for <search-for>...\n" + \
        "\n" + \
        "For more information try --help\n"

    # When/Then
    then_nonzero_status_code(context)

    # Then
    assert context.stderr == search_for_argument_missing_error


@then('printed is an error message detailing that the arguments ignore and only search are mutually exclusive.')
def then_only_and_ignore_search_mutually_exclusive(context):
    # Given
    mutually_exclusive_end = "\n" + \
        "USAGE:\n" + \
        "    unreferenced_files --search <search>... --search-for <search-for>... <--only-search <only-search>...|--ignore-search <ignore-search>...>\n" + \
        "\n" + \
        "For more information try --help\n"

    only_search_mutually_exclusive_error = "error: The argument '--only-search <only-search>...' cannot be used with one or more of the other specified arguments\n" + mutually_exclusive_end
    ignore_search_mutually_exclusive_error = "error: The argument '--ignore-search <ignore-search>...' cannot be used with one or more of the other specified arguments\n" + mutually_exclusive_end

    # When/Then
    then_nonzero_status_code(context)

    # Then
    assert context.stderr in [
        only_search_mutually_exclusive_error,
        ignore_search_mutually_exclusive_error]


@then('printed is an error message detailing that the arguments ignore and only search for are mutually exclusive.')
def then_only_and_ignore_search_for_mutually_exclusive(context):
    # Given
    mutually_exclusive_end = "\n" + \
        "USAGE:\n" + \
        "    unreferenced_files --search <search>... --search-for <search-for>... <--only-search-for <only-search-for>...|--ignore-search-for <ignore-search-for>...>\n" + \
        "\n" + \
        "For more information try --help\n"

    only_search_for_mutually_exclusive_error = "error: The argument '--only-search-for <only-search-for>...' cannot be used with one or more of the other specified arguments\n" + mutually_exclusive_end
    ignore_search_for_mutually_exclusive_error = "error: The argument '--ignore-search-for <ignore-search-for>...' cannot be used with one or more of the other specified arguments\n" + mutually_exclusive_end

    # When/Then
    then_nonzero_status_code(context)

    # Then
    assert context.stderr in [
        only_search_for_mutually_exclusive_error,
        ignore_search_for_mutually_exclusive_error]
