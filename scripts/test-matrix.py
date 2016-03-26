#!/usr/bin/env python3
# coding: utf-8

# Copyright ⓒ 2016 Daniel Keep.
#
# Licensed under the MIT license (see LICENSE or <http://opensource.org
# /licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
# <http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
# files in the project carrying such notice may not be copied, modified,
# or distributed except according to those terms.

import os.path
import subprocess
import sys
import yaml

TRACE = os.environ.get('TRACE_TEST_MATRIX', '') != ''
USE_ANSI = True if sys.platform != 'win32' else os.environ.get('FORCE_ANSI', '') != '' or os.environ.get('ConEmuANSI', 'OFF') == 'ON'

def msg(*args):
    if USE_ANSI: sys.stdout.write('\x1b[1;34m')
    sys.stdout.write('> ')
    if USE_ANSI: sys.stdout.write('\x1b[1;32m')
    for arg in args:
        sys.stdout.write(str(arg))
    if USE_ANSI: sys.stdout.write('\x1b[0m')
    sys.stdout.write('\n')
    sys.stdout.flush()

def msg_trace(*args):
    if TRACE:
        if USE_ANSI: sys.stderr.write('\x1b[1;31m')
        sys.stderr.write('$ ')
        if USE_ANSI: sys.stderr.write('\x1b[0m')
        for arg in args:
            sys.stderr.write(str(arg))
        sys.stderr.write('\n')
        sys.stderr.flush()

def sh(cmd, env=None, stdout=None, stderr=None, checked=True):
    msg_trace('sh(%r, env=%r)' % (cmd, env))
    try:
        subprocess.check_call(cmd, env=env, stdout=stdout, stderr=stderr, shell=True)
    except:
        msg_trace('FAILED!')
        if checked:
            raise
        else:
            return False
    if not checked:
        return True

def translate_script(script):
    parts = script.split("&&")
    return [p.strip() for p in parts]

def main():
    travis = yaml.load(open('.travis.yml'))
    script = translate_script(travis['script'])
    default_rust_vers = travis['rust']

    vers = set(default_rust_vers)
    include_vers = []
    exclude_vers = set()

    for arg in sys.argv[1:]:
        if arg in vers and arg not in include_vers:
            include_vers.append(arg)
        elif arg.startswith('-') and arg[1:] in vers:
            exclude_vers.add(arg[1:])
        else:
            msg("Don't know how to deal with argument `%s`." % arg)
            sys.exit(1)

    if include_vers == []:
        include_vers = default_rust_vers[:]

    rust_vers = [v for v in include_vers if v not in exclude_vers]
    msg('Tests will be run for: %s' % ', '.join(rust_vers))

    results = []
    for rust_ver in rust_vers:
        msg('Running tests for %s...' % rust_ver)
        target_dir = os.path.join('target', rust_ver)
        log_path = os.path.join('local', 'tests', '%s.log' % rust_ver)
        log_file = open(log_path, 'wt')
        success = True
        cmd_env = os.environ.copy()
        cmd_env['CARGO_TARGET_DIR'] = target_dir
        for cmd in script:
            cmd_str = '> multirust run %s %s' % (rust_ver, cmd)
            log_file.write(cmd_str)
            log_file.write("\n")
            success = sh(
                'multirust run %s %s' % (rust_ver, cmd),
                checked=False,
                stdout=log_file, stderr=log_file,
                env=cmd_env,
                )
            if not success:
                log_file.write('Command failed.\n')
                break
        msg('... ', 'OK' if success else 'Failed!')
        results.append((rust_ver, success))
        log_file.close()

    print("")

    msg('Results:')
    for rust_ver, success in results:
        msg('%s: %s' % (rust_ver, 'OK' if success else 'Failed!'))

if __name__ == '__main__':
    main()
