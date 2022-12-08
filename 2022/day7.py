from functools import reduce
import operator
import pprint

cwd = ['/']
file_system = {
    '/': { 'size': 0}, 'size': 0
}

{
    '/': {
        'size': 10,
        'dpwg': {},
        'dvwfscw': {},
        'hccpl': {},
        'jsgbg': {},
        'lhjmzsl': {},
        'mwvbpw.mng': 63532,
    }
}

def getFromDict(dataDict, mapList):
    return reduce(operator.getitem, mapList, dataDict)

def setInDict(dataDict, mapList, key, value):
    getFromDict(dataDict, mapList[:-1])[mapList[-1]][key] = value

def process_cd(target_dir):
    if target_dir in getFromDict(file_system, cwd):
        cwd.append(target_dir)
    elif target_dir == '..':
        cwd.pop()

def process_ls_content(line_contents):
    if line_contents[0] == 'dir':
        # file_system[line_contents[1]] = {}
        setInDict(file_system, cwd, line_contents[1], {'size': 0})
    else:
        # file_system[line_contents[1]] = line_contents[0]
        setInDict(file_system, cwd, line_contents[1], int(line_contents[0]))


def process_filesystem(subdir):
    for key in subdir.keys():
        if isinstance(subdir[key], dict):
            # is dictionary
            process_filesystem(subdir[key])
        elif key != 'size':
            # is file
            subdir['size'] = subdir['size'] + subdir[key]

def process_dir_size(subdir):
    for key in subdir.keys():
        if isinstance(subdir[key], dict):
            # is dictionary
            subdir['size'] += subdir['size'] + subdir[key]['size']
            process_dir_size(subdir[key])
        elif key != 'size':
            pass

def part1():
    with open('./data/day7.txt') as f:
        processing_ls = False
        for line in f.readlines():
            line = line.strip()
            line_components = line.split(' ')
            if processing_ls and line_components[0] != '$':
                process_ls_content(line_components)
            else:
                processing_ls = False
                if line_components[0] == '$':
                    if line_components[1] == 'cd':
                        process_cd(line_components[-1])
                    if line_components[1] == 'ls':
                        processing_ls = True
    
    process_filesystem(file_system)
    process_dir_size(file_system)
    pprint.pprint(file_system, indent=4)
            
            
def part2():
    with open('./data/day7.txt') as f:
        for line in f.readlines():
            line = line.strip()
            

if __name__ == '__main__':
    part1()
    # part2()
    
