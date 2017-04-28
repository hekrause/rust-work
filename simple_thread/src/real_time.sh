 #!/bin/bash
echo "Shell-script."
pgrep -f simple_thread  | while read line ; do sudo chrt -f -p 99 $line ; done &
pgrep -f cargo  | while read line ; do sudo chrt -f -p 99 $line ; done &
