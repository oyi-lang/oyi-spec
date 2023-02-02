files=`ls *.pho`

for file in $files
do
  make all
  echo -n "$file: "
  ./phoron < $file
  rm -f phoron
done
