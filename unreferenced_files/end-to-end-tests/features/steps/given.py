import os
import hashlib
from behave import given

from utilities import execute_command


def reset_context(context):
    context.behave_directory = os.getcwd()

    context.pre_command = ""
    context.unreferenced_files_path = context.behave_directory + \
        "/../../target/debug/unreferenced_files"
    context.arguments = ""


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(
        context, remote_repository, commit_hash):
    reset_context(context)

    remote_repository_md5 = hashlib.md5(remote_repository.encode())
    context.remote_repository_cache = "/tmp/" + remote_repository_md5.hexdigest()

    if not os.path.exists(context.remote_repository_cache):
        (exit_code, _, _) = execute_command("git clone " +
                                            remote_repository + " " + context.remote_repository_cache)
        assert exit_code == 0

    os.chdir(context.remote_repository_cache)

    (exit_code, _, _) = execute_command("git reset --hard origin/HEAD")
    assert exit_code == 0

    (exit_code, _, _) = execute_command("git checkout " + commit_hash)
    assert exit_code == 0

    os.chdir(context.behave_directory)
