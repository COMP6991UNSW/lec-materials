#!/bin/sh

echo '#!/bin/sh' > my_file.sh

for i in `seq 100`; do
    echo "echo '$i'" >> my_file.sh;
done

chmod +x my_file.sh
