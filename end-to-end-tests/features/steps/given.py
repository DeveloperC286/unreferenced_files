import os
import tempfile
from behave import given

from utilities import execute_command


def reset_context(context):
    context.behave_directory = os.getcwd()
    context.temporary_directory = tempfile.TemporaryDirectory()

    context.pre_command = ""
    context.unreferenced_files_path = context.behave_directory + \
        "/../target/debug/unreferenced_files"
    context.arguments = ""


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(
        context, remote_repository, commit_hash):
    reset_context(context)

    os.chdir(context.temporary_directory.name)
    (exit_code, _) = execute_command(
        "git clone --depth 1 " + remote_repository + " .")
    assert exit_code == 0
    (exit_code, _) = execute_command("git fetch --depth 1 origin " + commit_hash)
    assert exit_code == 0
    os.chdir(context.behave_directory)
