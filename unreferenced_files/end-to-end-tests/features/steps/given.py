import os
import hashlib
from behave import given

from utilities import execute_command
from assertions import assert_command_successful


def reset_context(context):
    context.behave_directory = os.getcwd()

    context.pre_command = ""
    context.unreferenced_files_path = f"{context.behave_directory}/../../target/debug/unreferenced_files"
    context.arguments = ""


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(context, remote_repository, commit_hash):
    reset_context(context)

    md5 = hashlib.md5(remote_repository.encode()).hexdigest()
    context.remote_repository_cache = f"/tmp/{md5}/{commit_hash}"

    if not os.path.exists(context.remote_repository_cache):
        result = execute_command(f"git clone {remote_repository} {context.remote_repository_cache}")
        assert_command_successful(result)

        os.chdir(context.remote_repository_cache)

        result = execute_command(f"git checkout {commit_hash}")
        assert_command_successful(result)

        os.chdir(context.behave_directory)
