--TEST--
dirs functions test on Darwin
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
string(%d) "/Users/%s"
string(%d) "/Users/%s/Library/Caches"
string(%d) "/Users/%s/Library/Application Support"
string(%d) "/Users/%s/Library/Application Support"
string(%d) "/Users/%s/Library/Application Support"
string(%d) "/Users/%s/Library/Application Support"
string(%d) "/Users/%s/Library/Application Support"
NULL
string(%d) "/Users/%s/Library/Preferences"
NULL
NULL
string(%d) "/Users/%s/Music"
string(%d) "/Users/%s/Desktop"
string(%d) "/Users/%s/Documents"
string(%d) "/Users/%s/Downloads"
string(%d) "/Users/%s/Library/Fonts"
string(%d) "/Users/%s/Pictures"
string(%d) "/Users/%s/Public"
NULL
string(%d) "/Users/%s/Movies"
--SKIPIF--
<?php if (PHP_OS_FAMILY !== 'Darwin') die('skip'); ?>
