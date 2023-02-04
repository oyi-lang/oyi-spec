files=`ls *.pho`

for file in $files
do
  make clean
  make all
  echo -n "$file: "
  ./phoron < $file
done
