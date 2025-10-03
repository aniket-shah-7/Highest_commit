#!/bin/bash

start=1
end=100000
batch_size=1000

for ((i=start; i<=end; i+=batch_size)); do
    batch_end=$((i + batch_size - 1))
    if [ $batch_end -gt $end ]; then
        batch_end=$end
    fi
    
    for ((j=i; j<=batch_end; j++)); do
        git commit --allow-empty -m "empty commit #$j" > /dev/null
    done
    
    echo "Created commits $i to $batch_end"
    git push -f
done

echo "Completed creating $end commits"