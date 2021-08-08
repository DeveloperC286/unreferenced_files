from behave import *


@when('the flag --assert-no-unreferenced-files is set.')
def set_assert_no_unreferenced_files(context):
    context.arguments += " --assert-no-unreferenced-files "


@when('the flag --only-file-name is set.')
def set_assert_no_unreferenced_files(context):
    context.arguments += " --only-file-name "


@when('the argument --search-for is provided as "{search_for}".')
def set_search_for(context, search_for):
    context.arguments += " --search-for " + search_for + " "


@when('the argument --only-search-for is provided as "{only_search_for}".')
def set_only_search(context, only_search_for):
    context.arguments += " --only-search-for " + only_search_for + " "


@when('the argument --ignore-search-for is provided as "{ignore_search_for}".')
def set_only_search(context, ignore_search_for):
    context.arguments += " --ignore-search-for " + ignore_search_for + " "


@when('the argument --search is provided as "{search}".')
def set_search(context, search):
    context.arguments += " --search " + search + " "


@when('the argument --only-search is provided as "{only_search}".')
def set_only_search(context, only_search):
    context.arguments += " --only-search " + only_search + " "


@when('the argument --ignore-search is provided as "{ignore_search}".')
def set_only_search(context, ignore_search):
    context.arguments += " --ignore-search " + ignore_search + " "