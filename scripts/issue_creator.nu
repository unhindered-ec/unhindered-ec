#!/usr/bin/env nu
# This script automatically creates issues for all failing clippy lints

def main [base_link?: string] {

if (which gh | is-empty) {
  print "This script requires the installation of the github cli. Please install it and try again.";
  exit 1;
}

if (which cargo | is-empty) {
  print "This script requires a working rust (with cargo) installation.";
  exit 1;
}
  
  
let repo = gh repo set-default -v;
print $"This script will automatically create issues in the selected repo: ($repo). Are you sure you wish to continue?";
print "Type [Yes I want to continue!]:";

let input = input;
if ($input != "Yes I want to continue!") {
  exit 1;
}

let default_base = gh repo view --json url --jq '.url';

let link_base = if ($base_link | is-empty) {
  print "No blob url provided to link issues to - defaulting to the current `main` branch -- this is not recommended as it can lead to broken links once files on main are changed"
  print "Do you wish to proceed? [y/N]:"
  if (input | str upcase) != "Y" {
    exit 1;
  }

  $"($default_base)/blob/main"
} else {
  $base_link
}


let found_issues = cargo clippy --all-targets --message-format=json
| from json -o
| where {|x| ($x.reason | str trim) == "compiler-message" }
| each {|x| $x | upsert rendered $x.message.rendered }
| uniq-by rendered
| group-by message.code.code?;

if ($found_issues | is-empty) {
  print "No lint failures found ... exiting";
  exit 0;
}

let issues = $found_issues | transpose | each {|x|

  let issues = $x.column1 
  | each {|issue| 
    let primary_span = $issue.message.spans | where $it.is_primary | get 0; 
    let link = if $primary_span != null { (  $"<a href=\"($link_base)/($primary_span.file_name)#L($primary_span.line_start)C($primary_span.column_start)-L($primary_span.line_end)C($primary_span.column_end)\">($primary_span.file_name)</a>" ) };
    let other_spans = $issue.message.spans | where not $it.is_primary | each {|span|
      $"- <a href=\"($link_base)/($span.file_name)#L($span.line_start)C($span.column_start)-L($span.line_end)C($span.column_end)\">($span.file_name) Line ($span.line_start)-($span.line_end)</a>"
    } | str join "\n";
    $"
<details>
<summary>
(match $issue.message.level {"warning" => "⚠️", "error" => "🛑", other => other}) ($issue.message.message)
 -- ($link)
</summary>

```txt
($issue.message.rendered | str trim)
```

(if not ($other_spans | is-empty ) {
$'Other spans: 
($other_spans)'
 })

</details>
    " | str trim }
  | str join "\n";

  {
    lint: $x.column0,
    title: $"Fix failures for lint ($x.column0)", 
    content: $'
# Clippy reports for lint ($x.column0)

Issues found:
(
  $issues
)
  '}
};

let created_issues = $issues | each {|issue| 
  print $"Creating issue for lint ($issue.lint) ...";

  $issue.content | (gh issue create -F - -t $issue.title -a "@me" -l automated-creation,clippy-lints) | url parse | get path | split row "/" | last
};

print $"Creating tracking issue ...";

let master_issue = {
  title: $"Tracking issue for fixing outstanding clippy lints",
  content: $"
The following clippy lints currently fail, and they should be fixed:
($created_issues | each {|issue| 
  $'- [ ] #($issue)'
 } | str join "\n" )
  "
}

$master_issue.content | gh issue create -F - -t $master_issue.title -a "@me" -l automated-creation,clippy-lints


}
