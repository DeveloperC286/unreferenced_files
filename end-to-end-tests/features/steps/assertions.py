import os
from behave import *

from util import execute_command


@then('unreferenced files are not found.')
def then_not_found(context):
    execute_unreferenced_files(context)
    assert int(context.exit_code) == 0
    assert context.stdout == "".encode('utf-8')


@then('the unreferenced files are "{unreferenced_files}".')
def then_found(context, unreferenced_files):
    execute_unreferenced_files(context)
    unreferenced_files = unreferenced_files.strip() \
        .strip('\"').replace("\\n", '\n').encode('utf-8')
    assert context.stdout == unreferenced_files


@then('the status code is non-zero.')
def then_non_zero_status_code(context):
    if not hasattr(context, 'exit_code'):
        execute_unreferenced_files(context)
    assert int(context.exit_code) != 0


def execute_unreferenced_files(context):
    os.chdir(context.temporary_directory.name)
    (context.exit_code, context.stdout) = execute_command(
        context.pre_command + context.unreferenced_files_path + context.arguments)
    os.chdir(context.behave_directory)
