#!/bin/bash

start=1
end=100000
batch_size=100

for ((i=start; i<=end; i+=batch_size)); do
    batch_end=$((i + batch_size - 1))
    if [ $batch_end -gt $end ]; then
        batch_end=$end
    fi
    
    for ((j=i; j<=batch_end; j++)); do
        git commit --allow-empty -m "empty commit #$j" > /dev/null 2>&1
    done
    
    echo "Created commits $i to $batch_end"
    
    # Push every 1000 commits and run gc
    if ((i % 1000 == 0)) || ((batch_end == end)); then
        git gc --auto
        git push -f
        echo "Pushed batch up to $batch_end"
    fi
done

echo "Completed creating $end commits"