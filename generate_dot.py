#!/usr/bin/env python3

# Read the input file and generate a DOT file
with open('data/inputs/11.txt', 'r') as f:
    lines = f.readlines()

# Start the DOT file
dot_content = ['digraph G {']
dot_content.append('    rankdir=LR;')
dot_content.append('    node [shape=circle, fontsize=8];')
dot_content.append('')
dot_content.append('    // Special nodes')
dot_content.append('    you [fillcolor=lightgreen, style=filled];')
dot_content.append('    out [fillcolor=lightcoral, style=filled];')
dot_content.append('    fft [fillcolor=lightbrown, style=filled];')
dot_content.append('    dac [fillcolor=lightblue, style=filled];')
dot_content.append('')
dot_content.append('    // Edges')

# Parse each line and create edges
for line in lines:
    line = line.strip()
    if not line:
        continue

    if ':' in line:
        source, targets = line.split(':', 1)
        source = source.strip()
        targets = targets.strip().split()

        for target in targets:
            dot_content.append(f'    {source} -> {target};')

dot_content.append('}')

# Write the DOT file
with open('data/inputs/11.dot', 'w') as f:
    f.write('\n'.join(dot_content))

print('DOT file generated: data/inputs/11.dot')
