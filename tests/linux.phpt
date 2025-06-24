--TEST--
dirs functions test on linux
--EXTENSIONS--
dirs
--FILE--
<?php
var_dump(home_dir());
var_dump(cache_dir());
var_dump(config_dir());
var_dump(config_dir());
var_dump(config_local_dir());
var_dump(data_dir());
var_dump(data_local_dir());
var_dump(executable_dir());
var_dump(preference_dir());
var_dump(runtime_dir());
var_dump(state_dir());
var_dump(audio_dir());
var_dump(desktop_dir());
var_dump(document_dir());
var_dump(download_dir());
var_dump(font_dir());
var_dump(picture_dir());
var_dump(public_dir());
var_dump(template_dir());
var_dump(video_dir());
?>
--EXPECTF--
string(%d) "/%s"
string(%d) "/%s/.cache"
string(%d) "/%s/.config"
string(%d) "/%s/.config"
string(%d) "/%s/.config"
string(%d) "/%s/.local/share"
string(%d) "/%s/.local/share"
string(%d) "/%s/.local/bin"
string(%d) "/%s/.config"
NULL
string(%d) "/%s/.local/state"
NULL
NULL
NULL
NULL
string(%d) "/%s/.local/share/fonts"
NULL
NULL
NULL
NULL
--SKIPIF--
<?php if (PHP_OS_FAMILY !== 'Linux') die('skip'); ?>
