### Header comment

## Examples from: https://projectfluent.org/fluent/guide/index.html

## Group comment
# Comment

hello = Hello, world!

# $title (String) - The title of the bookmark to remove.
remove-bookmark = Are you sure you want to remove { $title }?

# $title (String) - The title of the bookmark to remove.
remove-bookmark = Really remove { $title }?

-brand-name = Firefox
installing = Installing { -brand-name }.

opening-brace = This message features an opening curly brace: {"{"}.
closing-brace = This message features a closing curly brace: {"}"}.

blank-is-removed =     This message starts with no blanks.
blank-is-preserved = {"    "}This message starts with 4 spaces.

leading-bracket =
    This message has an opening square bracket
    at the beginning of the third line:
    {"["}.

attribute-how-to =
    To add an attribute to this messages, write
    {".attr = Value"} on a new line.
    .attr = An actual attribute (not part of the text value above)

# This is OK, but cryptic and hard to read and edit.
literal-quote1 = Text in {"\""}double quotes{"\""}.

# This is preferred. Just use the actual double quote character.
literal-quote2 = Text in "double quotes".

privacy-label = Privacy{"\u00A0"}Policy

# The dash character is an EM DASH but depending on the font face,
# it might look like an EN DASH.
which-dash1 = It's a dash—or is it?

# Using a Unicode escape sequence makes the intent clear.
which-dash2 = It's a dash{"\u2014"}or is it?

# This will work fine, but the codepoint can be considered
# cryptic by other translators.
tears-of-joy1 = {"\U01F602"}

# This is preferred. You can instantly see what the Unicode
# character used here is.
tears-of-joy2 = 😂

single = Text can be written in a single line.

multi = Text can also span multiple lines
    as long as each new line is indented
    by at least one space.

block =
    Sometimes it's more readable to format
    multiline text as a "block", which means
    starting it on a new line. All lines must
    be indented by at least one space.

leading-spaces =     This message's value starts with the word "This".
leading-lines =


    This message's value starts with the word "This".
    The blank lines under the identifier are ignored.

blank-lines =

    The blank line above this line is ignored.
    This is a second line of the value.

    The blank line above this line is preserved.

multiline1 =
    This message has 4 spaces of indent
        on the second line of its value.

#   denotes the indent common to all lines (removed from the value).
# · denotes the indent preserved in the final value.
multiline1 =
    This message has 4 spaces of indent
    ····on the second line of its value.

multiline2 =
    ··This message starts with 2 spaces on the first
    first line of its value. The first 4 spaces of indent
    are removed from all lines.

multiline3 = This message has 4 spaces of indent
    ····on the second line of its value. The first
    line is not considered indented at all.

# Same value as multiline3 above.
multiline4 =     This message has 4 spaces of indent
    ····on the second line of its value. The first
    line is not considered indented at all.

multiline5 = This message ends up having no indent
        on the second line of its value.

welcome = Welcome, { $user }!
unread-emails = { $user } has { $email-count } unread emails.

# $duration (Number) - The duration in seconds.
time-elapsed = Time elapsed: { $duration }s.

# $duration (Number) - The duration in seconds.
time-elapsed = Time elapsed: { NUMBER($duration, maximumFractionDigits: 0) }s.

menu-save = Save
help-menu-save = Click { menu-save } to save the file.

-brand-name = Firefox
installing = Installing { -brand-name }.

emails =
    { $unreadEmails ->
        [one] You have one unread email.
       *[two] You have two unread emails.
        [other] You have { $unreadEmails } unread emails.
    }

your-score =
    { NUMBER($score, minimumFractionDigits: 1) ->
        [0.0]   You scored zero points. What happened?
       *[0.5]   You scored half a point. What happened?
        [other] You scored { NUMBER($score, minimumFractionDigits: 1) } points.
    }

your-rank = { NUMBER($pos, type: "ordinal") ->
   [1] You finished first!
   [one] You finished {$pos}st
  *[two] You finished {$pos}nd
   [few] You finished {$pos}rd
   [other] You finished {$pos}th
}

login-input = Predefined value
    .placeholder = email@example.com
    .aria-label = Login input value
    .title = Type your login email

-brand-name = Firefox

about = About { -brand-name }.
update-successful = { -brand-name } has been updated.

# A contrived example to demonstrate how variables
# can be passed to terms.
-https = https://{ $host }
visit = Visit { -https(host: "example.com") } for more information.

-brand-name =
    { $case ->
       *[nominative] Firefox
        [locative] Firefoksie
    }

# "About Firefox."
about = Informacje o { -brand-name(case: "locative") }.

-brand-name =
    { $case ->
       *[nominative] Firefox
        [locative] Firefoksie
    }

# "Firefox has been successfully updated."
update-successful = { -brand-name } został pomyślnie zaktualizowany.

-brand-name = Aurora
    .gender = feminine

update-successful =
    { -brand-name.gender ->
        [masculine] { -brand-name } został zaktualizowany.
        [feminine] { -brand-name } została zaktualizowana.
       *[other] Program { -brand-name } został zaktualizowany.
    }

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

### Localization for Server-side strings of Firefox Screenshots

## Global phrases shared across pages

my-shots = My Shots
home-link = Home
screenshots-description =
    Screenshots made simple. Take, save, and
    share screenshots without leaving Firefox.

## Creating page

# Note: { $title } is a placeholder for the title of the web page
# captured in the screenshot. The default, for pages without titles, is
# creating-page-title-default.
creating-page-title = Creating { $title }
creating-page-title-default = page
creating-page-wait-message = Saving your shot…

emails = You have { $unreadEmails } unread emails.
emails2 = You have { NUMBER($unreadEmails) } unread emails.

last-notice =
    Last checked: { DATETIME($lastChecked, day: "numeric", month: "long") }.

today-is = Today is { DATETIME($date) }

dpi-ratio = Your DPI ratio is { NUMBER($ratio, minimumFractionDigits: 2) }

today-is = Today is { DATETIME($date, month: "long", year: "numeric", day: "numeric") }

emails = Number of unread emails { $unreadEmails }

emails2 = Number of unread emails { NUMBER($unreadEmails) }

liked-count = { $num ->
        [0]     No likes yet.
        [one]   One person liked your message
       *[other] { $num } people liked your message
    }

liked-count2 = { NUMBER($num) ->
        [0]     No likes yet.
        [one]   One person liked your message
       *[other] { $num } people liked your message
    }

log-time = Entry time: { $date }

log-time2 = Entry time: { DATETIME($date) }
no-args = Entry time: { DATETIME() }

today = Today is { $day }

today = Today is { DATETIME($day, weekday: "short") }

block-placeable =

{ "string literal" }

attribute-reference = { today.day }

-term-reference = { today.day }

inline-expression-number-literal = { 1 }

inline-expression-number-literal = { -123 }

inline-expression-number-literal = { 3.14 }

inline-expression-inline-placeable = { { 123 } }

plain-message-with-attributes =
    .one = One
    .two = Two

message-pattern-first = { "message pattern first" }
