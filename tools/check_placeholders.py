#!/usr/bin/env python3
"""Check that every answer placeholder still points at the same code after a translation.

For each task-info.yaml with placeholders, the script compares the base revision
(default: `master`) with the working tree:

  * the file still has the same number of placeholders, with the same lengths;
  * the text at [offset, offset + length) is identical in both versions;
  * the code before the placeholder is identical once comments are ignored, so the
    placeholder sits in the same context and not on another copy of the same text.

Offsets are counted in UTF-16 code units, like the JetBrains Academy plugin does
(Java strings), not in bytes and not in Python characters.

Usage:
    python3 tools/check_placeholders.py [--base REV] [PATH ...]

PATH limits the check to task-info.yaml files under the given directories or files.
The script exits with a non-zero status if at least one placeholder does not match.
Requires PyYAML.
"""
import argparse
import glob
import os
import subprocess
import sys

import yaml

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def utf16_slice(s, off, ln):
    b = s.encode("utf-16-le")
    return b[off * 2:(off + ln) * 2].decode("utf-16-le")


def utf16_to_index(s, off):
    """Convert a UTF-16 offset into a Python string index (None if it splits a surrogate pair)."""
    units = 0
    for i, ch in enumerate(s):
        if units == off:
            return i
        if units > off:
            return None
        units += 2 if ord(ch) > 0xFFFF else 1
    return len(s) if units == off else None


# --- Rust lexer: splits source code into code and comment segments ----------------

def _is_ident_char(ch):
    return ch.isalnum() or ch == "_"


def rust_segments(src):
    """Return a list of (kind, text) where kind is 'code' or 'comment'.

    Handles line comments (//, ///, //!), nested block comments, string literals with
    escapes, raw strings (r"..", r#".."#, br"..", cr".."), byte strings, char literals
    and lifetimes, so that `//` inside a string is not mistaken for a comment.
    """
    segs = []
    n = len(src)
    i = 0
    code_start = 0

    def flush(upto):
        if upto > code_start:
            segs.append(("code", src[code_start:upto]))

    while i < n:
        c = src[i]
        if c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            j = n if j == -1 else j
            flush(i)
            segs.append(("comment", src[i:j]))
            i = code_start = j
            continue
        if c == "/" and i + 1 < n and src[i + 1] == "*":
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif src.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            flush(i)
            segs.append(("comment", src[i:j]))
            i = code_start = j
            continue
        prev_ident = i > 0 and _is_ident_char(src[i - 1])
        if c in "rbc" and not prev_ident:
            # raw strings: r"...", r#"..."#, br"...", cr"..."
            k = i + 1
            if c in "bc" and k < n and src[k] == "r":
                k += 1
            if c == "r" or (k == i + 2):
                h = k
                while h < n and src[h] == "#":
                    h += 1
                if h < n and src[h] == '"':
                    closing = '"' + "#" * (h - k)
                    j = src.find(closing, h + 1)
                    i = n if j == -1 else j + len(closing)
                    continue
            if c in "bc" and i + 1 < n and src[i + 1] in "\"'":
                i += 1  # b"..", c"..", b'.': handled by the quote branches below
                continue
        if c == '"':
            j = i + 1
            while j < n:
                if src[j] == "\\":
                    j += 2
                    continue
                if src[j] == '"':
                    break
                j += 1
            i = j + 1
            continue
        if c == "'":
            if i + 1 < n and src[i + 1] == "\\":
                j = i + 3  # skip the quote, the backslash and the escaped character
                while j < n and src[j] != "'":
                    j += 2 if src[j] == "\\" else 1
                i = j + 1
                continue
            if i + 2 < n and src[i + 2] == "'":
                i += 3
                continue
            i += 1  # lifetime or label
            continue
        i += 1
    flush(n)
    return segs


def comment_skeleton(text, path):
    """Text with the content of every comment removed (line structure kept).

    Only .rs files are lexed; any other file is returned unchanged, as those files
    are not supposed to change at all.
    """
    if not path.endswith(".rs"):
        return text
    out = []
    for kind, seg in rust_segments(text):
        if kind == "code":
            out.append(seg)
        elif seg.startswith("//"):
            out.append("//")
        else:
            out.append("/*" + "\n" * seg.count("\n") + "*/")
    return "".join(out)


# --- git helpers ----------------------------------------------------------------------

def git_show(rev, relpath):
    r = subprocess.run(["git", "show", f"{rev}:{relpath}"], cwd=ROOT, capture_output=True)
    if r.returncode != 0:
        return None
    return r.stdout.decode("utf-8")


def find_task_infos(paths):
    if not paths:
        paths = [ROOT]
    found = set()
    for p in paths:
        p = os.path.abspath(p)
        if os.path.isfile(p):
            if os.path.basename(p) == "task-info.yaml":
                found.add(p)
            continue
        for f in glob.glob(os.path.join(glob.escape(p), "**", "task-info.yaml"), recursive=True):
            found.add(os.path.abspath(f))
    return sorted(os.path.relpath(f, ROOT) for f in found)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--base", default="master", help="git revision with the original files (default: master)")
    ap.add_argument("-v", "--verbose", action="store_true", help="print every checked placeholder")
    ap.add_argument("paths", nargs="*", help="directories or task-info.yaml files to check (default: whole repo)")
    args = ap.parse_args()

    errors = []
    checked = 0
    for info_rel in find_task_infos(args.paths):
        task_dir = os.path.dirname(info_rel)
        with open(os.path.join(ROOT, info_rel), encoding="utf-8") as fh:
            new_info = yaml.safe_load(fh) or {}
        old_text = git_show(args.base, info_rel)
        if old_text is None:
            continue
        old_info = yaml.safe_load(old_text) or {}
        new_files = {f["name"]: f for f in new_info.get("files") or []}
        for old_file in old_info.get("files") or []:
            old_ph = old_file.get("placeholders") or []
            if not old_ph:
                continue
            name = old_file["name"]
            where = f"{info_rel} [{name}]"
            new_file = new_files.get(name)
            if new_file is None:
                errors.append(f"{where}: file entry is missing in the new task-info.yaml")
                continue
            new_ph = new_file.get("placeholders") or []
            if len(new_ph) != len(old_ph):
                errors.append(f"{where}: {len(old_ph)} placeholders in {args.base}, {len(new_ph)} now")
                continue
            src_rel = os.path.join(task_dir, name)
            old_src = git_show(args.base, src_rel)
            try:
                with open(os.path.join(ROOT, src_rel), encoding="utf-8") as fh:
                    new_src = fh.read()
            except OSError as e:
                errors.append(f"{where}: cannot read {src_rel}: {e}")
                continue
            if old_src is None:
                errors.append(f"{where}: {src_rel} does not exist in {args.base}")
                continue
            for idx, (op, np_) in enumerate(zip(old_ph, new_ph), 1):
                checked += 1
                tag = f"{where} placeholder #{idx}"
                if op["length"] != np_["length"]:
                    errors.append(f"{tag}: length changed {op['length']} -> {np_['length']}")
                    continue
                old_region = utf16_slice(old_src, op["offset"], op["length"])
                new_region = utf16_slice(new_src, np_["offset"], np_["length"])
                if old_region != new_region:
                    errors.append(f"{tag}: text differs\n    {args.base} @{op['offset']}: {old_region!r}\n"
                                  f"    now @{np_['offset']}: {new_region!r}")
                    continue
                oi = utf16_to_index(old_src, op["offset"])
                ni = utf16_to_index(new_src, np_["offset"])
                if oi is None or ni is None:
                    errors.append(f"{tag}: offset splits a UTF-16 surrogate pair")
                    continue
                if comment_skeleton(old_src[:oi], name) != comment_skeleton(new_src[:ni], name):
                    errors.append(f"{tag}: same text but different context (code before the placeholder "
                                  f"differs), offset {op['offset']} -> {np_['offset']}")
                    continue
                if args.verbose:
                    print(f"ok  {tag}: {op['offset']} -> {np_['offset']} {old_region[:40]!r}")
    for e in errors:
        print("FAIL " + e)
    print(f"Checked {checked} placeholders, {len(errors)} problem(s).")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
