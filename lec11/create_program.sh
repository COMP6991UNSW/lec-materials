#!/bin/sh

rm -f generated.sh;
echo '#!/bin/sh' > generated.sh;
chmod +x generated.sh;

for i in `seq 10`; do
	echo "echo 'Number: $i';" >> generated.sh;
done
