#!/bin/sh

echo '#!/bin/sh' >program.sh

for i in $(seq 10);
do
	echo "echo '$i'" >>program.sh
done

chmod +x program.sh
