TITLE: Implementing index.js for a New Locale in date-fns
DESCRIPTION: This snippet shows how to create the index.js file for a new locale in date-fns. It includes importing necessary functions, setting locale metadata, and exporting the locale object.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/i18nContributionGuide.md#2025-04-17_snippet_0

LANGUAGE: javascript
CODE:
```
import formatDistance from "./_lib/formatDistance/index.js";
import formatLong from "./_lib/formatLong/index.js";
import formatRelative from "./_lib/formatRelative/index.js";
import localize from "./_lib/localize/index.js";
import match from "./_lib/match/index.js";

/**
 * @type {Locale}
 * @category Locales
 *
 * // Name of the locale.
 * // Inside the parentheses - name of the country - if the locale uses the four letter code, e.g. en-US, fr-CA or pt-BR.
 * @summary English locale (United States).
 *
 * // Name of the language (used by https://date-fns.org/ website)
 * @language English
 *
 * // ISO 639-2 code. See the list here:
 * // https://www.loc.gov/standards/iso639-2/php/code_list.php
 * // Used by https://date-fns.org/ to detect the list of the countries that uses the language.
 * @iso-639-2 eng
 *
 * // Authors of the locale (including anyone who corrected or fixed the locale)
 * @author Sasha Koss [@kossnocorp]{@link https://github.com/kossnocorp}
 * @author Lesha Koss [@leshakoss]{@link https://github.com/leshakoss}
 */
var locale = {
  code: "en",
  formatDistance: formatDistance,
  formatLong: formatLong,
  formatRelative: formatRelative,
  localize: localize,
  match: match,
  options: {
    // Index of the first day of the week.
    // Sunday is 0, Monday is 1, Saturday is 6.
    weekStartsOn: 0,

    // Nth of January which is always in the first week of the year. See:
    // https://en.wikipedia.org/wiki/Week#The_ISO_week_date_system
    // http://www.pjh2.de/datetime/weeknumber/wnd.php?l=en
    firstWeekContainsDate: 1,
  },
};

export default locale;
```

----------------------------------------

TITLE: Importing and Using date-fns for Date Formatting and Comparison in JavaScript
DESCRIPTION: This example demonstrates how to import specific functions from date-fns, format a date in 'yyyy-MM-dd' pattern, and sort an array of dates in ascending order using the compareAsc function.
SOURCE: https://github.com/date-fns/date-fns/blob/main/README.md#2025-04-17_snippet_0

LANGUAGE: javascript
CODE:
```
import { compareAsc, format } from "date-fns";

format(new Date(2014, 1, 11), "yyyy-MM-dd");
//=> '2014-02-11'

const dates = [
  new Date(1995, 6, 2),
  new Date(1987, 1, 11),
  new Date(1989, 6, 10),
];
dates.sort(compareAsc);
//=> [
//   Wed Feb 11 1987 00:00:00,
//   Mon Jul 10 1989 00:00:00,
//   Sun Jul 02 1995 00:00:00
// ]
```

----------------------------------------

TITLE: Creating an en-GB Locale by Extending en-US in date-fns
DESCRIPTION: This example demonstrates how to create a new locale (en-GB) by reusing components from an existing locale (en-US) while only implementing unique properties. The implementation imports shared formatting functions from en-US and adds GB-specific formatting and options.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/i18nContributionGuide.md#2025-04-17_snippet_22

LANGUAGE: javascript
CODE:
```
// Same as en-US
import formatDistance from "../en-US/_lib/formatDistance/index.js";
import formatRelative from "../en-US/_lib/formatRelative/index.js";
import localize from "../en-US/_lib/localize/index.js";
import match from "../en-US/_lib/match/index.js";

// Unique for en-GB
import formatLong from "./_lib/formatLong/index.js";

/**
 * @type {Locale}
 * @category Locales
 * @summary English locale (United Kingdom).
 * @language English
 * @iso-639-2 eng
 * @author John Doe [@example]{@link https://github.com/example}
 */
var locale = {
  formatDistance: formatDistance,
  formatLong: formatLong,
  formatRelative: formatRelative,
  localize: localize,
  match: match,

  // Unique for en-GB
  options: {
    weekStartsOn: 1,
    firstWeekContainsDate: 4,
  },
};

export default locale;
```

----------------------------------------

TITLE: Importing Functions from date-fns Submodules in JavaScript
DESCRIPTION: This snippet shows how to import functions from the main date-fns module and its FP (functional programming) submodule.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/gettingStarted.md#2025-04-17_snippet_1

LANGUAGE: javascript
CODE:
```
// The main submodule:
import { addDays } from "date-fns";

// FP variation:
import { addDays, format } from "date-fns/fp";
```

----------------------------------------

TITLE: Creating a Locale-Aware Format Wrapper for date-fns
DESCRIPTION: Shows how to create a wrapper function that simplifies locale switching when formatting dates. The wrapper imports multiple locales, stores the current locale in a global variable, and applies it automatically to all date formatting operations.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/i18n.md#2025-04-17_snippet_1

LANGUAGE: javascript
CODE:
```
// app/_lib/format.js

import { format } from "date-fns";
import { enGB, eo, ru } from "date-fns/locale";

const locales = { enGB, eo, ru };

// by providing a default string of 'PP' or any of its variants for `formatStr`
// it will format dates in whichever way is appropriate to the locale
export default function (date, formatStr = "PP") {
  return format(date, formatStr, {
    locale: locales[window.__localeId__], // or global.__localeId__
  });
}

// Later:

import format from "app/_lib/format";

window.__localeId__ = "enGB";
format(friday13, "EEEE d");
//=> 'Friday 13'

window.__localeId__ = "eo";
format(friday13, "EEEE d");
//=> 'vendredo 13'

// If the format string is omitted, it will take the default for the locale.
window.__localeId__ = "enGB";
format(friday13);
//=> Jul 13, 2019

window.__localeId__ = "eo";
format(friday13);
//=> 2019-jul-13
```

----------------------------------------

TITLE: Using formatDistance and subDays Functions from date-fns in JavaScript
DESCRIPTION: This snippet shows how to import and use the formatDistance and subDays functions from date-fns to calculate and format the difference between two dates.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/gettingStarted.md#2025-04-17_snippet_3

LANGUAGE: javascript
CODE:
```
import { formatDistance, subDays } from "date-fns";

formatDistance(subDays(new Date(), 3), new Date(), { addSuffix: true });
//=> "3 days ago"
```

----------------------------------------

TITLE: Formatting and Comparing Dates with date-fns in JavaScript
DESCRIPTION: This snippet demonstrates how to use the format and compareAsc functions from date-fns to format a date and sort an array of dates in ascending order.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/gettingStarted.md#2025-04-17_snippet_0

LANGUAGE: javascript
CODE:
```
import { format, compareAsc } from "date-fns";

format(new Date(2014, 1, 11), "MM/dd/yyyy");
//=> '02/11/2014'

const dates = [
  new Date(1995, 6, 2),
  new Date(1987, 1, 11),
  new Date(1989, 6, 10),
];
dates.sort(compareAsc);
//=> [
//   Wed Feb 11 1987 00:00:00,
//   Mon Jul 10 1989 00:00:00,
//   Sun Jul 02 1995 00:00:00
// ]
```

----------------------------------------

TITLE: Formatting Relative Date Representations
DESCRIPTION: Generates human-readable relative date strings based on proximity to a reference date
SOURCE: https://github.com/date-fns/date-fns/blob/main/src/locale/en-US/snapshot.md#2025-04-17_snippet_3

LANGUAGE: markdown
CODE:
```
## `formatRelative`

If now is January 1st, 2000, 00:00.
```

----------------------------------------

TITLE: Installing date-fns via npm
DESCRIPTION: This snippet shows the npm command to install date-fns as a project dependency. The --save flag ensures the dependency is added to package.json.
SOURCE: https://github.com/date-fns/date-fns/blob/main/README.md#2025-04-17_snippet_1

LANGUAGE: bash
CODE:
```
npm install date-fns --save
```

----------------------------------------

TITLE: Using TZDate Extension with date-fns Functions
DESCRIPTION: Demonstrates how to use TZDate extension to handle DST transitions correctly across different time zones. Shows the difference between system time zone and specified time zone behavior.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/timeZones.md#2025-04-17_snippet_0

LANGUAGE: typescript
CODE:
```
import { TZDate } from "@date-fns/tz";
import { addHours } from "date-fns";

// Given that the system time zone is America/Los_Angeles
// where DST happens on Sunday, 13 March 2022, 02:00:00

// Using the system time zone will produce 03:00 instead of 02:00 because of DST:
const date = new Date(2022, 2, 13);
addHours(date, 2).toString();
//=> 'Sun Mar 13 2022 03:00:00 GMT-0700 (Pacific Daylight Time)'

// Using Asia/Singapore will provide the expected 02:00:
const tzDate = new TZDate(2022, 2, 13, "Asia/Singapore");
addHours(tzDate, 2).toString();
//=> 'Sun Mar 13 2022 02:00:00 GMT+0800 (Singapore Standard Time)'
```

----------------------------------------

TITLE: Transposing Date Values Between Time Zones
DESCRIPTION: Shows how to transpose date values between different time zones while maintaining the same local time using the transpose function.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/timeZones.md#2025-04-17_snippet_4

LANGUAGE: typescript
CODE:
```
import { transpose } from "date-fns";
import { tz } from "@date-fns/tz";

// Singapore is the system time zone:
const sgDate = new Date(2024, 8 /* Sep */, 7, 6, 5, 4);
//=> 'Wed Sep 07 2024 06:05:04 GMT+0800 (Singapore Standard Time)'

// Transpose the date to Los Angeles time zone:
const laDate = transpose(sgDate, tz("America/Los_Angeles"));
//=> 'Wed Sep 07 2024 06:05:04 GMT-0700 (Pacific Daylight Time)'

// Transpose back to local time zone using Date:
const systemDate = transpose(laDate, Date);
//=> 'Wed Sep 07 2024 06:05:04 GMT+0800 (Singapore Standard Time)'
```

----------------------------------------

TITLE: Matching Localized Values (en-US)
DESCRIPTION: This snippet defines the `match` object for the `en-US` locale, used by the `parse` function.  It includes regular expressions and utility functions for matching and parsing ordinal numbers, eras, quarters, months, days, and day periods. The `buildMatchPatternFn` and `buildMatchFn` functions are assumed to be imported from other modules within the library.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/i18nContributionGuide.md#2025-04-17_snippet_15

LANGUAGE: javascript
CODE:
```
```js
// In `en-US` locale:
import buildMatchPatternFn from "../../../_lib/buildMatchPatternFn/index.js";
import buildMatchFn from "../../../_lib/buildMatchFn/index.js";

var matchOrdinalNumberPattern = /^(\d+)(th|st|nd|rd)?/i;
var parseOrdinalNumberPattern = /\d+/i;

var matchEraPatterns = {
  narrow: /^(b|a)/i,
  abbreviated: /^(b\.?\s?c\.?|b\.?\s?c\.?\s?e\.?|a\.?\s?d\.?|c\.?\s?e\.?)/i,
  wide: /^(before christ|before common era|anno domini|common era)/i,
};
var parseEraPatterns = {
  any: [/^b/i, /^(a|c)/i],
};

var matchQuarterPatterns = {
  narrow: /^[1234]/i,
  abbreviated: /^q[1234]/i,
  wide: /^[1234](th|st|nd|rd)? quarter/i,
};
var parseQuarterPatterns = {
  any: [/1/i, /2/i, /3/i, /4/i],
};

var matchMonthPatterns = {
  narrow: /^[jfmasond]/i,
  abbreviated: /^(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)/i,
  wide: /^(january|february|march|april|may|june|july|august|september|october|november|december)/i,
};
var parseMonthPatterns = {
  narrow: [
    /^j/i,
    /^f/i,
    /^m/i,
    /^a/i,
    /^m/i,
    /^j/i,
    /^j/i,
    /^a/i,
    /^s/i,
    /^o/i,
    /^n/i,
    /^d/i,
  ],
  any: [
    /^ja/i,
    /^f/i,
    /^mar/i,
    /^ap/i,
    /^may/i,
    /^jun/i,
    /^jul/i,
    /^au/i,
    /^s/i,
    /^o/i,
    /^n/i,
    /^d/i,
  ],
};

var matchDayPatterns = {
  narrow: /^[smtwf]/i,
  short: /^(su|mo|tu|we|th|fr|sa)/i,
  abbreviated: /^(sun|mon|tue|wed|thu|fri|sat)/i,
  wide: /^(sunday|monday|tuesday|wednesday|thursday|friday|saturday)/i,
};
var parseDayPatterns = {
  narrow: [/^s/i, /^m/i, /^t/i, /^w/i, /^t/i, /^f/i, /^s/i],
  any: [/^su/i, /^m/i, /^tu/i, /^w/i, /^th/i, /^f/i, /^sa/i],
};

var matchDayPeriodPatterns = {
  narrow: /^(a|p|mi|n|(in the|at) (morning|afternoon|evening|night))/i,
  any: /^([ap]\.?\s?m\.?|midnight|noon|(in the|at) (morning|afternoon|evening|night))/i,
};
var parseDayPeriodPatterns = {
  any: {
    am: /^a/i,
    pm: /^p/i,
    midnight: /^mi/i,
    noon: /^no/i,
    morning: /morning/i,
    afternoon: /afternoon/i,
    evening: /evening/i,
    night: /night/i,
  },
};

var match = {
  ordinalNumber: buildMatchPatternFn({
    matchPattern: matchOrdinalNumberPattern,
    parsePattern: parseOrdinalNumberPattern,
    valueCallback: function (value) {
      return parseInt(value, 10);
    },
  }),

  era: buildMatchFn({
    matchPatterns: matchEraPatterns,
    defaultMatchWidth: "wide",
    parsePatterns: parseEraPatterns,
    defaultParseWidth: "any",
  }),

  quarter: buildMatchFn({
    matchPatterns: matchQuarterPatterns,
    defaultMatchWidth: "wide",
    parsePatterns: parseQuarterPatterns,
    defaultParseWidth: "any",
    valueCallback: function (index) {
      return index + 1;
    },
  }),

  month: buildMatchFn({
    matchPatterns: matchMonthPatterns,
    defaultMatchWidth: "wide",
    parsePatterns: parseMonthPatterns,
    defaultParseWidth: "any",
  }),

  day: buildMatchFn({
    matchPatterns: matchDayPatterns,
    defaultMatchWidth: "wide",
    parsePatterns: parseDayPatterns,
    defaultParseWidth: "any",
  }),

  dayPeriod: buildMatchFn({
    matchPatterns: matchDayPeriodPatterns,
    defaultMatchWidth: "any",
    parsePatterns: parseDayPeriodPatterns,
    defaultParseWidth: "any",
  }),
};

export default match;
```
```

----------------------------------------

TITLE: Comparing Wrong vs Correct Date Formatting in JavaScript
DESCRIPTION: Demonstrates the correct and incorrect usage of date formatting tokens, specifically contrasting YYYY-MM-DD vs yyyy-MM-dd format patterns and D.MM.YY vs d.MM.yy parsing patterns.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/unicodeTokens.md#2025-04-17_snippet_0

LANGUAGE: javascript
CODE:
```
// ❌ Wrong!
format(new Date(), "YYYY-MM-DD");
//=> 2018-10-283

// ✅ Correct
format(new Date(), "yyyy-MM-dd");
//=> 2018-10-10

// ❌ Wrong!
parse("11.02.87", "D.MM.YY", new Date()).toString();
//=> 'Sat Jan 11 1986 00:00:00 GMT+0200 (EET)'

// ✅ Correct
parse("11.02.87", "d.MM.yy", new Date()).toString();
//=> 'Wed Feb 11 1987 00:00:00 GMT+0200 (EET)'
```

----------------------------------------

TITLE: Implementing era Localization Function for en-US Locale in date-fns
DESCRIPTION: This snippet demonstrates how to implement the era localization function for the en-US locale in date-fns using the buildLocalizeFn helper function.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/i18nContributionGuide.md#2025-04-17_snippet_3

LANGUAGE: javascript
CODE:
```
// In `en-US` locale:
import buildLocalizeFn from "../../../_lib/buildLocalizeFn/index.js";

var eraValues = {
  narrow: ["B", "A"],
  abbreviated: ["BC", "AD"],
  wide: ["Before Christ", "Anno Domini"],
};

var localize = {
  // ...
  era: buildLocalizeFn({
    values: eraValues,
    defaultWidth: "wide",
  }),
  // ...
};

export default localize;
```

----------------------------------------

TITLE: Mixing Different Time Zone Date Objects
DESCRIPTION: Shows how to work with multiple date objects in different time zones for business day calculations. Demonstrates how date-fns normalizes arguments based on the reference type.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/timeZones.md#2025-04-17_snippet_1

LANGUAGE: typescript
CODE:
```
import { TZDate } from "@date-fns/tz";
import { differenceInBusinessDays } from "date-fns";

const laterDate = new TZDate(2025, 0, 1, "Asia/Singapore");
const earlierDate = new TZDate(2024, 0, 1, "America/New_York");

// Will calculate in Asia/Singapore
differenceInBusinessDays(laterDate, earlierDate);
//=> 262

// Will calculate in America/New_York
differenceInBusinessDays(earlierDate, laterDate);
//=> -261
```

----------------------------------------

TITLE: Using Locales with date-fns formatDistance Function
DESCRIPTION: Demonstrates how to import and use a specific locale (Esperanto) with the formatDistance function. Shows how to pass the locale as an option to properly format date differences in different languages.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/i18n.md#2025-04-17_snippet_0

LANGUAGE: javascript
CODE:
```
import { formatDistance } from "date-fns";
// Require Esperanto locale
import { eo } from "date-fns/locale";

const result = formatDistance(
  new Date(2016, 7, 1),
  new Date(2015, 0, 1),
  { locale: eo }, // Pass the locale as an option
);
//=> 'pli ol 1 jaro'
```

----------------------------------------

TITLE: Formatting Relative Dates (ru)
DESCRIPTION: This snippet provides a more complex implementation of `formatRelative` for the `ru` locale, handling grammatical genders and cases. It includes helper functions `lastWeek`, `thisWeek`, and `nextWeek` to format dates relative to the current week.  The `formatRelative` function selects and applies the appropriate format based on the token and date context.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/i18nContributionGuide.md#2025-04-17_snippet_14

LANGUAGE: javascript
CODE:
```
```javascript
// In `ru` locale
var accusativeWeekdays = [
  "воскресенье",
  "понедельник",
  "вторник",
  "среду",
  "четверг",
  "пятницу",
  "субботу",
];

function lastWeek(day) {
  var weekday = accusativeWeekdays[day];

  switch (day) {
    case 0:
      return "'в прошлое " + weekday + " в' p";
    case 1:
    case 2:
    case 4:
      return "'в прошлый " + weekday + " в' p";
    case 3:
    case 5:
    case 6:
      return "'в прошлую " + weekday + " в' p";
  }
}

function thisWeek(day) {
  // ...
}

function nextWeek(day) {
  // ...
}

var formatRelativeLocale = {
  lastWeek: function (date, baseDate, options) {
    var day = date.getDay();
    if (isSameUTCWeek(date, baseDate, options)) {
      return thisWeek(day);
    } else {
      return lastWeek(day);
    }
  },
  yesterday: "'вчера в' p",
  today: "'сегодня в' p",
  tomorrow: "'завтра в' p",
  nextWeek: function (date, baseDate, options) {
    var day = date.getDay();
    if (isSameUTCWeek(date, baseDate, options)) {
      return thisWeek(day);
    } else {
      return nextWeek(day);
    }
  },
  other: "P",
};

export default function formatRelative(token, date, baseDate, options) {
  var format = formatRelativeLocale[token];

  if (typeof format === "function") {
    return format(date, baseDate, options);
  }

  return format;
}
```
```

----------------------------------------

TITLE: Using date-fns with All Locales Bundle
DESCRIPTION: Shows how to load the main date-fns module along with all locales bundle for internationalization support.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/cdn.md#2025-04-17_snippet_3

LANGUAGE: html
CODE:
```
<script src="https://cdn.jsdelivr.net/npm/date-fns@3.6.0/cdn.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/date-fns@3.6.0/locale/cdn.min.js"></script>
<script>
  dateFns.formatRelative(dateFns.subDays(new Date(), 3), new Date(), {
    locale: dateFns.locale.es,
  });
  //=> "el viernes pasado a las 19:26"
</script>
```

----------------------------------------

TITLE: Using Additional Token Options in JavaScript Date Formatting
DESCRIPTION: Shows how to properly use D, DD, YY, and YYYY tokens with the useAdditionalDayOfYearTokens and useAdditionalWeekYearTokens options.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/unicodeTokens.md#2025-04-17_snippet_1

LANGUAGE: javascript
CODE:
```
format(new Date(), "D", { useAdditionalDayOfYearTokens: true });
//=> '283'

parse("365+1987", "DD+YYYY", new Date(), {
  useAdditionalDayOfYearTokens: true,
  useAdditionalWeekYearTokens: true,
}).toString();
//=> 'Wed Dec 31 1986 00:00:00 GMT+0200 (EET)'
```

----------------------------------------

TITLE: Formatting Relative Dates (en-US)
DESCRIPTION: This snippet defines the `formatRelative` function and the `formatRelativeLocale` object for the `en-US` locale. It specifies how relative date tokens like 'lastWeek', 'yesterday', 'today', 'tomorrow', 'nextWeek', and 'other' should be formatted. The function returns the corresponding format string based on the provided token.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/i18nContributionGuide.md#2025-04-17_snippet_13

LANGUAGE: javascript
CODE:
```
```javascript
// In `en-US` locale
var formatRelativeLocale = {
  lastWeek: "'last' eeee 'at' p",
  yesterday: "'yesterday at' p",
  today: "'today at' p",
  tomorrow: "'tomorrow at' p",
  nextWeek: "eeee 'at' p",
  other: "P",
};

export default function formatRelative(token, date, baseDate, options) {
  return formatRelativeLocale[token];
}
```
```

----------------------------------------

TITLE: Installing date-fns via npm or yarn in Bash
DESCRIPTION: This snippet demonstrates how to install the date-fns package using either npm or yarn package managers.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/gettingStarted.md#2025-04-17_snippet_2

LANGUAGE: bash
CODE:
```
npm install date-fns --save
# or
yarn add date-fns
```

----------------------------------------

TITLE: Using formatDuration in date-fns for JavaScript
DESCRIPTION: The formatDuration function formats a duration object into a human-readable string. It properly handles singular/plural units and can format durations with different time components.
SOURCE: https://github.com/date-fns/date-fns/blob/main/src/locale/en-NZ/snapshot.md#2025-04-17_snippet_12

LANGUAGE: javascript
CODE:
```
formatDuration(duration, [options])
```

----------------------------------------

TITLE: Basic FP Usage with date-fns
DESCRIPTION: Demonstrates basic usage of FP functions from date-fns including currying, formatting with options, and array operations. Shows how to use addYears and formatWithOptions with locale support.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/fp.md#2025-04-17_snippet_0

LANGUAGE: javascript
CODE:
```
import { addYears, formatWithOptions } from "date-fns/fp";
import { eo } from "date-fns/locale";
import toUpper from "lodash/fp/toUpper"; // 'date-fns/fp' is compatible with 'lodash/fp'!

// If FP function has not received enough arguments, it returns another function
const addFiveYears = addYears(5);

// Several arguments can be curried at once
const dateToString = formatWithOptions({ locale: eo }, "d MMMM yyyy");

const dates = [
  new Date(2017, 0 /* Jan */, 1),
  new Date(2017, 1 /* Feb */, 11),
  new Date(2017, 6 /* Jul */, 2),
];

const formattedDates = dates.map(addFiveYears).map(dateToString).map(toUpper);
//=> ['1 JANUARO 2022', '11 FEBRUARO 2022', '2 JULIO 2022']
```

----------------------------------------

TITLE: Basic date-fns CDN Usage
DESCRIPTION: Shows how to load the main date-fns module via CDN and use basic date manipulation functions.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/cdn.md#2025-04-17_snippet_1

LANGUAGE: html
CODE:
```
<script src="https://cdn.jsdelivr.net/npm/date-fns@3.6.0/cdn.min.js"></script>
<script>
  dateFns.addDays(new Date(2014, 1, 11), 10);
  //=> Tue Feb 21 2014 00:00:00
</script>
```

----------------------------------------

TITLE: Formatting Date Distances with date-fns in JavaScript
DESCRIPTION: This code snippet illustrates the expected outputs when calculating the distance between a fixed date (January 1st, 2000) and various date inputs. It demonstrates how to include seconds and suffixes in the results.
SOURCE: https://github.com/date-fns/date-fns/blob/main/src/locale/en-CA/snapshot.md#2025-04-17_snippet_9

LANGUAGE: JavaScript
CODE:
```
| Date                     | Result             | `includeSeconds: true` | `addSuffix: true`      |
| ------------------------ | ------------------ | ---------------------- | ---------------------- |
| 2006-01-01T00:00:00.000Z | about 6 years      | about 6 years          | in about 6 years       |
| 2005-01-01T00:00:00.000Z | about 5 years      | about 5 years          | in about 5 years       |
| 2004-01-01T00:00:00.000Z | about 4 years      | about 4 years          | in about 4 years       |
| 2003-01-01T00:00:00.000Z | about 3 years      | about 3 years          | in about 3 years       |
| 2002-01-01T00:00:00.000Z | about 2 years      | about 2 years          | in about 2 years       |
| 2001-06-01T00:00:00.000Z | over a year        | over a year            | in over a year         |
| 2001-02-01T00:00:00.000Z | about a year       | about a year           | in about a year        |
| 2001-01-01T00:00:00.000Z | about a year       | about a year           | in about a year        |
| 2000-06-01T00:00:00.000Z | 5 months           | 5 months               | in 5 months            |
| 2000-03-01T00:00:00.000Z | 2 months           | 2 months               | in 2 months            |
| 2000-02-01T00:00:00.000Z | about a month      | about a month          | in about a month       |
| 2000-01-15T00:00:00.000Z | 14 days            | 14 days                | in 14 days             |
| 2000-01-02T00:00:00.000Z | a day              | a day                  | in a day               |
| 2000-01-01T06:00:00.000Z | about 6 hours      | about 6 hours          | in about 6 hours       |
| 2000-01-01T01:00:00.000Z | about an hour      | about an hour          | in about an hour       |
| 2000-01-01T00:45:00.000Z | about an hour      | about an hour          | in about an hour       |
| 2000-01-01T00:30:00.000Z | 30 minutes         | 30 minutes             | in 30 minutes          |
| 2000-01-01T00:15:00.000Z | 15 minutes         | 15 minutes             | in 15 minutes          |
| 2000-01-01T00:01:00.000Z | a minute           | a minute               | in a minute            |
| 2000-01-01T00:00:25.000Z | less than a minute | half a minute          | in less than a minute  |
| 2000-01-01T00:00:15.000Z | less than a minute | less than 20 seconds   | in less than a minute  |
| 2000-01-01T00:00:05.000Z | less than a minute | less than 10 seconds   | in less than a minute  |
| 2000-01-01T00:00:00.000Z | less than a minute | less than 5 seconds    | less than a minute ago |
| 1999-12-31T23:59:55.000Z | less than a minute | less than 10 seconds   | less than a minute ago |
| 1999-12-31T23:59:45.000Z | less than a minute | less than 20 seconds   | less than a minute ago |
| 1999-12-31T23:59:35.000Z | less than a minute | half a minute          | less than a minute ago |
| 1999-12-31T23:59:00.000Z | a minute           | a minute               | a minute ago           |
| 1999-12-31T23:45:00.000Z | 15 minutes         | 15 minutes             | 15 minutes ago         |
| 1999-12-31T23:30:00.000Z | 30 minutes         | 30 minutes             | 30 minutes ago         |
| 1999-12-31T23:15:00.000Z | about an hour      | about an hour          | about an hour ago      |
| 1999-12-31T23:00:00.000Z | about an hour      | about an hour          | about an hour ago      |
| 1999-12-31T18:00:00.000Z | about 6 hours      | about 6 hours          | about 6 hours ago      |
| 1999-12-30T00:00:00.000Z | 2 days             | 2 days                 | 2 days ago             |
| 1999-12-15T00:00:00.000Z | 17 days            | 17 days                | 17 days ago            |
| 1999-12-01T00:00:00.000Z | about a month      | about a month          | about a month ago      |
| 1999-11-01T00:00:00.000Z | 2 months           | 2 months               | 2 months ago           |
| 1999-06-01T00:00:00.000Z | 7 months           | 7 months               | 7 months ago           |
| 1999-01-01T00:00:00.000Z | about a year       | about a year           | about a year ago       |
| 1998-12-01T00:00:00.000Z | about a year       | about a year           | about a year ago       |
| 1998-06-01T00:00:00.000Z | over a year        | over a year            | over a year ago        |
| 1998-01-01T00:00:00.000Z | about 2 years      | about 2 years          | about 2 years ago      |
| 1997-01-01T00:00:00.000Z | about 3 years      | about 3 years          | about 3 years ago      |
| 1996-01-01T00:00:00.000Z | about 4 years      | about 4 years          | about 4 years ago      |
| 1995-01-01T00:00:00.000Z | about 5 years      | about 5 years          | about 5 years ago      |
| 1994-01-01T00:00:00.000Z | about 6 years      | about 6 years          | about 6 years ago      |
```

----------------------------------------

TITLE: Date Format Token Examples in Markdown Table
DESCRIPTION: Reference table showing date formatting tokens and their output patterns for both format and parse functions. Demonstrates formatting for calendar years, week-numbering years, quarters, and months with various token patterns.
SOURCE: https://github.com/date-fns/date-fns/blob/main/src/locale/nb/snapshot.md#2025-04-17_snippet_0

LANGUAGE: markdown
CODE:
```
| Title                           | Token string | Date                     | `format` result                                      | `parse` result           |
| ------------------------------- | ------------ | ------------------------ | ---------------------------------------------------- | ------------------------ |
| Calendar year                   | yo           | 1987-02-11T12:13:14.015Z | 1987.                                                | 1987-01-01T00:00:00.000Z |
|                                 |              | 0005-01-01T12:13:14.015Z | 5.                                                   | 0005-01-01T00:00:00.000Z |
```

----------------------------------------

TITLE: Using Multiple Locales with date-fns CDN
DESCRIPTION: Demonstrates loading date-fns core and multiple locale files via CDN and using them for relative date formatting in different languages.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/cdn.md#2025-04-17_snippet_0

LANGUAGE: html
CODE:
```
<script src="https://cdn.jsdelivr.net/npm/date-fns@3.6.0/cdn.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/date-fns@3.6.0/locale/es/cdn.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/date-fns@3.6.0/locale/ru/cdn.min.js"></script>
<script>
  dateFns.formatRelative(dateFns.subDays(new Date(), 3), new Date());
  //=> "last Friday at 7:26 p.m."

  dateFns.formatRelative(dateFns.subDays(new Date(), 3), new Date(), {
    locale: dateFns.locale.es,
  });
  //=> "el viernes pasado a las 19:26"

  dateFns.formatRelative(dateFns.subDays(new Date(), 3), new Date(), {
    locale: dateFns.locale.ru,
  });
  //=> "в прошлую пятницу в 19:26"
</script>
```

----------------------------------------

TITLE: Using 'in' Option for Time Zone Context
DESCRIPTION: Demonstrates how to use the 'in' option to explicitly specify the time zone context for calculations.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/timeZones.md#2025-04-17_snippet_3

LANGUAGE: typescript
CODE:
```
import { tz } from "@date-fns/tz";

// Will calculate in Asia/Singapore
differenceInBusinessDays(laterDate, earlierDate);
//=> 262

// Will normalize to America/Los_Angeles
differenceInBusinessDays(laterDate, earlierDate, {
  in: tz("America/Los_Angeles"),
});
//=> 261
```

----------------------------------------

TITLE: Formatting with Custom Localized Date and Time - JavaScript
DESCRIPTION: This snippet customizes date formatting to display a combination of day, month names, and time in Portuguese. It processes standard date strings and alters the output to fit local user preferences. This is particularly useful for applications catering to Portuguese-speaking users.
SOURCE: https://github.com/date-fns/date-fns/blob/main/src/locale/pt-BR/snapshot.md#2025-04-17_snippet_3

LANGUAGE: JavaScript
CODE:
```
|                                 | PPpp         | 1987-01-11T12:13:14.015Z | 11 jan 1987, 12:13:14                                       | 1987-01-11T12:13:14.000Z |
```

----------------------------------------

TITLE: Date Format Pattern Documentation Table
DESCRIPTION: A markdown table showing various date formatting patterns and their outputs. Includes examples for hours (0-23, 0-11, 1-24), minutes, seconds, and different levels of localized date formatting (P, PP, PPP, PPPP).
SOURCE: https://github.com/date-fns/date-fns/blob/main/src/locale/gd/snapshot.md#2025-04-17_snippet_2

LANGUAGE: markdown
CODE:
```
| Hour [0-23]                     | Ho           | 2019-02-11T11:13:14.015Z | 11mh                                                            | 2019-02-11T11:00:00.000Z |
|                                 |              | 2019-02-11T23:13:14.015Z | 23mh                                                            | 2019-02-11T23:00:00.000Z |
| Hour [0-11]                     | Ko           | 2019-02-11T11:13:14.015Z | 11mh                                                            | 2019-02-11T11:00:00.000Z |
|                                 |              | 2019-02-11T23:13:14.015Z | 11mh                                                            | 2019-02-11T23:00:00.000Z |
| Hour [1-24]                     | ko           | 2019-02-11T11:13:14.015Z | 11mh                                                            | 2019-02-11T11:00:00.000Z |
|                                 |              | 2019-02-11T23:13:14.015Z | 23mh                                                            | 2019-02-11T23:00:00.000Z |
```

----------------------------------------

TITLE: Time Zone String Representation
DESCRIPTION: Illustrates how dates are represented when converted between different time zones using the withTimeZone method.
SOURCE: https://github.com/date-fns/date-fns/blob/main/docs/timeZones.md#2025-04-17_snippet_2

LANGUAGE: typescript
CODE:
```
laterDate.withTimeZone("Asia/Singapore").toString();
//=> 'Wed Jan 01 2025 00:00:00 GMT+0800 (Singapore Standard Time)'
earlyDate.withTimeZone("Asia/Singapore").toString();
//=> 'Mon Jan 01 2024 13:00:00 GMT+0800 (Singapore Standard Time)'

laterDate.withTimeZone("America/New_York").toString();
//=> 'Tue Dec 31 2024 11:00:00 GMT-0500 (Eastern Standard Time)'
earlyDate.withTimeZone("America/New_York").toString();
//=> 'Mon Jan 01 2024 00:00:00 GMT-0500 (Eastern Standard Time)'
```
