files=`ls *.pho`

for file in $files
do
  make -s clean
  make -s all
  echo -n "$file: "
  ./phoron < $file
done
