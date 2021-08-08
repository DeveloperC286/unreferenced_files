import os
from behave import *

from utilities import execute_unreferenced_files

MISSING_SEARCH = "error: The following required arguments were not provided:\n    --search <search>...\n\n"
MISSING_SEARCH_FOR = "error: The following required arguments were not provided:\n    --search-for <search-for>...\n\n"
ONLY_SEARCH_MUTUALLY_EXCLUSIVE = "error: The argument '--only-search <only-search>...' cannot be used with one or more of the other specified arguments\n"
IGNORE_SEARCH_MUTUALLY_EXCLUSIVE = "error: The argument '--ignore-search <ignore-search>...' cannot be used with one or more of the other specified arguments\n"
ONLY_SEARCH_FOR_MUTUALLY_EXCLUSIVE = "error: The argument '--only-search-for <only-search-for>...' cannot be used with one or more of the other specified arguments\n"
IGNORE_SEARCH_FOR_MUTUALLY_EXCLUSIVE = "error: The argument '--ignore-search-for <ignore-search-for>...' cannot be used with one or more of the other specified arguments\n"


@then('unreferenced files are not found.')
def then_unreferenced_files_not_found(context):
    execute_unreferenced_files(context)
    assert int(context.exit_code) == 0
    assert context.stdout == "".encode('utf-8')


@then('the unreferenced files are "{unreferenced_files}".')
def then_unreferenced_files_found(context, unreferenced_files):
    execute_unreferenced_files(context)
    unreferenced_files = unreferenced_files.strip() \
        .strip('\"').replace("\\n", '\n').encode('utf-8')
    assert context.stdout == unreferenced_files


@then('the status code is nonzero.')
def then_nonzero_status_code(context):
    if not hasattr(context, 'exit_code'):
        execute_unreferenced_files(context)
    assert int(context.exit_code) != 0


@then('printed is an error message detailing that the argument search is missing.')
def then_search_argument_missing_error(context):
    execute_unreferenced_files(context)
    assert starts_with(context.stdout, MISSING_SEARCH)
    then_nonzero_status_code(context)


@then('printed is an error message detailing that the argument search for is missing.')
def then_search_for_argument_missing_error(context):
    execute_unreferenced_files(context)
    assert starts_with(context.stdout, MISSING_SEARCH_FOR)
    then_nonzero_status_code(context)


@then('printed is an error message detailing that the arguments ignore and only search are mutually exclusive.')
def then_only_and_ignore_search_mutually_exclusive(context):
    execute_unreferenced_files(context)
    assert starts_with(
        context.stdout,
        ONLY_SEARCH_MUTUALLY_EXCLUSIVE) or starts_with(
        context.stdout,
        IGNORE_SEARCH_MUTUALLY_EXCLUSIVE)
    then_nonzero_status_code(context)


@then('printed is an error message detailing that the arguments ignore and only search for are mutually exclusive.')
def then_only_and_ignore_search_for_mutually_exclusive(context):
    execute_unreferenced_files(context)
    assert starts_with(
        context.stdout,
        ONLY_SEARCH_FOR_MUTUALLY_EXCLUSIVE) or starts_with(
        context.stdout,
        IGNORE_SEARCH_FOR_MUTUALLY_EXCLUSIVE)
    then_nonzero_status_code(context)


def execute_unreferenced_files(context):
    os.chdir(context.temporary_directory.name)
    (context.exit_code, context.stdout) = execute_command(
        context.pre_command + context.unreferenced_files_path + context.arguments)
    os.chdir(context.behave_directory)


def starts_with(stdout, error_message):
    return stdout.strip().startswith(bytes(error_message.strip(), 'utf-8'))
