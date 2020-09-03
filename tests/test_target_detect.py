"""Tests for automatic target detection."""

from rust_test_common import *

# TODO
# - manual config targets


class TestTargetDetect(TestBase):

    def test_multi_targets(self):
        """Test automatic target detection."""

        expected_targets = [
            # Exact target name matches.
            ('src/lib.rs', [('src/lib.rs', '--lib')]),
            ('src/main.rs', [('src/main.rs', '--bin multi-targets')]),
            ('src/bin/bin1.rs', [('src/bin/bin1.rs', '--bin bin1')]),
            ('src/bin/bin2.rs', [('src/bin/bin2.rs', '--bin bin2')]),
            ('src/altmain.rs', [('src/altmain.rs', '--bin otherbin')]),
            ('examples/ex1.rs', [('examples/ex1.rs', '--example ex1')]),
            ('examples/ex2.rs', [('examples/ex2.rs', '--example ex2')]),
            ('tests/test1.rs', [('tests/test1.rs', '--test test1')]),
            ('tests/test2.rs', [('tests/test2.rs', '--test test2')]),
            ('benches/bench1.rs', [('benches/bench1.rs', '--bench bench1')]),
            ('benches/bench2.rs', [('benches/bench2.rs', '--bench bench2')]),
            # Random module in src/, defaults to --lib.
            ('src/lmod1.rs', [('src/lib.rs', '--lib')]),
            # Target failure.  Perhaps this should run rustc directly?
            ('mystery.rs', []),
            ('build.rs', []),
            # Shared module in test, not possible to easily determine which
            # test it belongs to.
            ('tests/common/helpers.rs', [('tests/test1.rs', '--test test1'),
                                         ('tests/test2.rs', '--test test2'),
                                         ('tests/test_context.rs', '--test test_context')]),
            # proc-macro kind
            ('pmacro/src/lib.rs', [('pmacro/src/lib.rs', '--lib')]),
            # Different lib types.
            ('libs/cdylib/src/lib.rs', [('libs/cdylib/src/lib.rs', '--lib')]),
            ('libs/dylib/src/lib.rs', [('libs/dylib/src/lib.rs', '--lib')]),
            ('libs/rlib/src/lib.rs', [('libs/rlib/src/lib.rs', '--lib')]),
            ('libs/staticlib/src/lib.rs', [('libs/staticlib/src/lib.rs', '--lib')]),
        ]
        rustc_version = util.get_rustc_version(sublime.active_window(),
                                               plugin_path)
        if semver.match(rustc_version, '>=1.17.0'):
            # Example libraries had issues before 1.17.
            expected_targets.extend([
                ('examples/exlib.rs',[('examples/exlib.rs', '--example exlib')]),
                ('examples/excdylib.rs',[('examples/excdylib.rs', '--example excdylib')]),
            ])

        for (path, targets) in expected_targets:
            path = os.path.join('tests', 'multi-targets', path)
            targets = [(os.path.normpath(
                os.path.join(plugin_path, 'tests', 'multi-targets', x[0])),
                x[1].split()) for x in targets]
            self._with_open_file(path,
                lambda view: self._test_multi_targets(view, targets))

    def _test_multi_targets(self, view, expected_targets):
        expected_targets.sort()
        # Check the targets match expectation.
        t = target_detect.TargetDetector(view.window())
        targets = t.determine_targets(view.file_name())
        targets.sort()
        self.assertEqual(targets, expected_targets)
