import os
from behave import *

from util import execute_command


@when('the flag --assert-no-unreferenced-files is set.')
def set_assert_no_unreferenced_files(context):
    context.arguments += " --assert-no-unreferenced-files "


@when('the argument --from is provided as "{from_dir}".')
def set_from(context, from_dir):
    context.arguments += " --from " + from_dir + " "


@when('the argument --search is provided as "{search_dir}".')
def set_from(context, search_dir):
    context.arguments += " --search " + search_dir + " "


def execute_unreferenced_files(context):
    (context.exit_code, context.stdout) = execute_command(
        context.pre_command + context.unreferenced_files_path + context.arguments)
    os.chdir(context.behave_directory)


@then('unreferenced files are not found.')
def then_not_found(context):
    execute_unreferenced_files(context)
    assert int(context.exit_code) == 0
    assert context.stdout == "".encode('utf-8')


@then('the unreferenced files are "{unreferenced_files}".')
def then_found(context, unreferenced_files):
    execute_unreferenced_files(context)
    assert int(context.exit_code) != 0
    unreferenced_files = unreferenced_files.strip() \
        .strip('\"').replace("\\n", '\n').encode('utf-8')
    assert context.stdout == unreferenced_files
