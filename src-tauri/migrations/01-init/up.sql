CREATE TABLE "window_lifetime" IF NOT EXISTS (
    "title" TEXT, -- window title
    "app" TEXT, -- app id
    "start" INTEGER, -- timestamp of window open
    "end" INTEGER, -- timestamp of window close
    "window_id" INTEGER -- wayland id of window
) STRICT;

CREATE TABLE "window_focus_lifetime" IF NOT EXISTS (
	"title" TEXT,
	"app" TEXT,
	"start" INTEGER,
	"end" INTEGER,
	"window_id" INTEGER
) STRICT;

CREATE TABLE "website_lifetime" IF NOT EXISTS (
	"title" TEXT,
	"url" TEXT,
	"start" INTEGER,
	"end" INTEGER
) STRICT;

CREATE TABLE "website_focus_lifetime" IF NOT EXISTS (
	"title" TEXT,
	"url" TEXT,
	"start" INTEGER,
	"end" INTEGER
) STRICT;

CREATE TABLE "audio_track_lifetime" IF NOT EXISTS (
    "title" TEXT,
    "artist" TEXT,
    "app" TEXT,
    "audio_length" INTEGER,
    "start" INTEGER,
    "end" INTEGER,
    "track_id" TEXT
) STRICT;

CREATE TABLE "user_idle_lifetime" IF NOT EXISTS (
	"start" INTEGER, -- start timestamp of user idleness
	"end" INTEGER -- end timestamp of user idleness
) STRICT;

CREATE TABLE "activity_category" IF NOT EXISTS (
	"name" TEXT NOT NULL,
	"description" TEXT NOT NULL
) STRICT;

CREATE TABLE "daily_limit" IF NOT EXISTS (
    "start" TEXT, -- ISO 8601 limit start time
    "end" TEXT, -- ISO 8601 limit end time
    "applies_cron" TEXT, -- cron segments for day/month, month, day/week for weekly or monthly scheduling
    "duration" INTEGER NOT NULL, -- unix timestamp for number of seconds of limit
    "allow_bypass" BOOLEAN,
) STRICT;

CREATE TABLE "cooldown_limit" IF NOT EXISTS (
	"start" TEXT, -- ISO 8601 limit start time
    "end" TEXT, -- ISO 8601 limit end time
    "applies_cron" TEXT, -- cron segments for day/month, month, day/week for weekly or monthly scheduling
    "time_threshold" INTEGER NOT NULL, -- unix timestamp for number of seconds of limit
    "time_window" INTEGER, -- window of time in which active time must exceed the threshold, in seconds
    "time_cooldown" INTEGER NOT NULL, -- amount of time that hourlasso blocks usage for
    "randomize_unblock" BOOLEAN, -- randomize the unblock time to prevent people waiting on the limit to finish
    "watch_penalty" BOOLEAN, -- whether to lengthen the cooldown time if the user doesn't leave their computer
    "allow_bypass" BOOLEAN,
) STRICT;

CREATE TABLE "off_limits_time" IF NOT EXISTS (
	"start" TEXT, -- ISO 8601 time when off limits time starts
	"end" TEXT, -- ISO 8601 end time
	"applies_cron" TEXT, -- cron segments for day/month, month, day/week for weekly or monthly scheduling
	"name" TEXT, -- name of the off limits time block
	"enabled" BOOLEAN, -- if the user has enabled the off limits time block
	"message" TEXT, -- message displayed to the user during this
	"sub_message" TEXT, -- secondary message
	"allow_bypass" BOOLEAN
) STRICT;


