import os
import tempfile
from behave import given
from util import execute_command


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
        "git clone " + remote_repository + " .")
    assert exit_code == 0
    (exit_code, _) = execute_command("git checkout " + commit_hash)
    assert exit_code == 0
    os.chdir(context.behave_directory)


@given('the directory is changed to the cloned repository.')
def change_into_git_dir(context):
    os.chdir(context.temporary_directory.name)
